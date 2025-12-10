//! Mesh, Material, Texture, and pipeline resources for rendering geometry
//! that's expected to change every frame.

mod dynamic_buffer;
mod frame_constants;
mod material;
mod mesh;
mod texture;
pub(crate) mod utility;

use {
    self::{frame_constants::FrameConstants, mesh::Mesh},
    crate::graphics::vulkan::{
        raii, spirv_words, Frame, FramesInFlight, VulkanContext,
    },
    anyhow::{Context, Result},
    ash::vk,
    dynamic_buffer::DynamicBuffer,
    material::Material,
    std::sync::Arc,
};

pub use self::{
    mesh::{TrianglesMesh, Vertex},
    texture::{Texture, TextureAtlas, TextureLoader},
};

const INITIAL_CAPACITY: usize = 4096;

#[derive(Debug, Clone)]
struct DrawParams {
    index_offset: u32,
    vertex_offset: u32,
    index_count: u32,
    material: Arc<Material>,
    transform_index: u32,
    scissor: vk::Rect2D,
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
struct MeshTransform {
    matrix: [[f32; 4]; 4],
}

/// All of the resources required to assemble draw commands for a frame.
///
/// The renderer keeps one instance per frame-in-flight so resources can be
/// updated freely while constructing the frame.
struct FrameDraw {
    vertex_buffer: DynamicBuffer<Vertex>,
    index_buffer: DynamicBuffer<u32>,
    transforms: DynamicBuffer<MeshTransform>,
    draw_params: Vec<DrawParams>,
}

impl FrameDraw {
    pub fn new(ctx: &VulkanContext) -> Result<Self> {
        Ok(Self {
            vertex_buffer: DynamicBuffer::new(
                ctx,
                INITIAL_CAPACITY,
                vk::BufferUsageFlags::STORAGE_BUFFER
                    | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            )?,
            index_buffer: DynamicBuffer::new(
                ctx,
                INITIAL_CAPACITY,
                vk::BufferUsageFlags::INDEX_BUFFER,
            )?,
            transforms: DynamicBuffer::new(
                ctx,
                INITIAL_CAPACITY,
                vk::BufferUsageFlags::STORAGE_BUFFER
                    | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            )?,
            draw_params: Vec::with_capacity(4),
        })
    }
}

/// A renderer optimized for streaming new vertex data to the GPU every frame.
pub struct StreamingRenderer<PerFrameDataT: Copy = ()> {
    frame_draw_resources: Vec<FrameDraw>,
    frame_constants: FrameConstants<PerFrameDataT>,

    pipeline_layout: raii::PipelineLayout,

    default_vertex_shader_module: raii::ShaderModule,
    default_fragment_shader_module: raii::ShaderModule,
    default_material: Arc<Material>,
    image_format: vk::Format,
}

impl<PerFrameDataT: Copy> StreamingRenderer<PerFrameDataT> {
    pub fn new(
        ctx: &VulkanContext,
        image_format: vk::Format,
        frames_in_flight: &FramesInFlight,
        texture_atlas: &TextureAtlas,
    ) -> Result<Self> {
        let frame_constants = FrameConstants::new(ctx, frames_in_flight)
            .context("Unable to create FrameData instance")?;

        // create pipeline resources
        let pipeline_layout = Material::create_pipeline_layout(
            ctx,
            texture_atlas.descriptor_set_layout(),
            frame_constants.descriptor_set_layout(),
        )
        .context("Unable to create pipeline layout")?;

        let frame_draw_resources = {
            let mut resources =
                Vec::with_capacity(frames_in_flight.frame_count());
            for frame_index in 0..frames_in_flight.frame_count() {
                resources.push(FrameDraw::new(ctx).context(format!(
                    "Unable to create draw resources for frame {}",
                    frame_index
                ))?);
            }
            resources
        };

        let default_vertex_shader_module = {
            let vertex_shader_words =
                spirv_words(include_bytes!("./shaders/triangle.vert.spv"))
                    .context("Unable to pack default vertex shader source")?;
            raii::ShaderModule::new(
                "DefaultVertexShader",
                ctx.device.clone(),
                &vk::ShaderModuleCreateInfo {
                    code_size: vertex_shader_words.len() * 4,
                    p_code: vertex_shader_words.as_ptr(),
                    ..Default::default()
                },
            )
            .context("Unable to create default vertex shader module")?
        };
        let default_fragment_shader_module = {
            let fragment_shader_words =
                spirv_words(include_bytes!("./shaders/triangle.frag.spv"))
                    .context("Unable to pack default fragment shader source")?;
            raii::ShaderModule::new(
                "DefaultFragmentShader",
                ctx.device.clone(),
                &vk::ShaderModuleCreateInfo {
                    code_size: fragment_shader_words.len() * 4,
                    p_code: fragment_shader_words.as_ptr(),
                    ..Default::default()
                },
            )
            .context("Unable to create default fragment shader module")?
        };
        let default_material = Arc::new(
            Material::new(
                ctx,
                image_format,
                &pipeline_layout,
                &default_vertex_shader_module,
                &default_fragment_shader_module,
            )
            .context("Unable to create default material")?,
        );

        Ok(Self {
            frame_draw_resources,
            frame_constants,

            pipeline_layout,
            default_vertex_shader_module,
            default_fragment_shader_module,
            default_material,
            image_format,
        })
    }

    /// Creates a new rendering material. See the documentation for [Material]
    /// for details on allowed shader inputs and outputs.
    ///
    /// Default vertex and fragment shaders are used automatically if either
    /// is omitted.
    pub fn new_material(
        &self,
        ctx: &VulkanContext,
        vertex_shader: Option<&raii::ShaderModule>,
        fragment_shader: Option<&raii::ShaderModule>,
    ) -> Result<Arc<Material>> {
        let material = Material::new(
            ctx,
            self.image_format,
            &self.pipeline_layout,
            vertex_shader.unwrap_or(&self.default_vertex_shader_module),
            fragment_shader.unwrap_or(&self.default_fragment_shader_module),
        )
        .context("Unable to create new material!")?;
        Ok(Arc::new(material))
    }

    /// Returns the default material for use by meshes without special material
    /// requirements.
    pub fn default_material(&self) -> &Arc<Material> {
        &self.default_material
    }

    /// Prepares the meshes for this frame.
    ///
    /// This should only be called once per frame, calling it repeatedly will
    /// only render whatever meshes were provided last.
    pub fn prepare_meshes(
        &mut self,
        ctx: &VulkanContext,
        frame: &Frame,
        meshes: &[&dyn Mesh],
    ) -> Result<()> {
        let frame_draw = &mut self.frame_draw_resources[frame.frame_index()];
        frame_draw.draw_params.clear();

        // collect the vertex and index references and assemble the draw params
        let (vertex_data, index_data) = {
            let mut vertex_data = Vec::with_capacity(meshes.len());
            let mut index_data = Vec::with_capacity(meshes.len());
            let mut index_offset = 0;
            let mut vertex_offset = 0;

            for (transform_index, mesh) in meshes.iter().enumerate() {
                let vertices = mesh.vertices();
                let indices = mesh.indices();
                vertex_data.push(vertices);
                index_data.push(indices);

                frame_draw.draw_params.push(DrawParams {
                    index_offset,
                    vertex_offset,
                    index_count: indices.len() as u32,
                    material: mesh.material().clone(),
                    transform_index: transform_index as u32,
                    scissor: mesh.scissor(),
                });

                index_offset += indices.len() as u32;
                vertex_offset += vertices.len() as u32;
            }

            (vertex_data, index_data)
        };

        // write mesh data into frame-specific buffers
        unsafe {
            frame_draw
                .vertex_buffer
                .write_chunked_data(ctx, &vertex_data)
                .context("Unable to write frame vertex data!")?;
            frame_draw
                .index_buffer
                .write_chunked_data(ctx, &index_data)
                .context("Unable to write index data!")?;
            frame_draw.transforms.write_iterated_data(
                ctx,
                meshes.iter().map(|mesh| MeshTransform {
                    matrix: mesh.transform().data.0,
                }),
            )?;
        }

        Ok(())
    }

    pub fn set_frame_constants(
        &mut self,
        frame: &Frame,
        data: PerFrameDataT,
    ) -> Result<()> {
        self.frame_constants.set_data(frame, data)
    }

    /// Binds the texture atlas for the frame.
    ///
    /// This only needs to be done once a frame, regardless of how many meshes
    /// there are as mesh pipelines are required to have compatible pipeline
    /// layouts.
    pub fn bind_texture_atlas(
        &mut self,
        ctx: &VulkanContext,
        frame: &Frame,
        texture_atlas: &TextureAtlas,
    ) {
        unsafe {
            ctx.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                0,
                &[texture_atlas.descriptor_set()],
                &[],
            );
        }
    }

    /// Emits draw commands for all of the meshes in the current frame.
    ///
    /// NOTE: it is incorrect to call this multiple times for the same frame as
    ///       there is only one internal vertex buffer per frame.
    pub fn write_draw_commands(
        &mut self,
        ctx: &VulkanContext,
        frame: &Frame,
    ) -> Result<()> {
        let frame_draw = &mut self.frame_draw_resources[frame.frame_index()];
        unsafe {
            ctx.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                1,
                &[self.frame_constants.descriptor_set_for_frame(frame)],
                &[],
            );
            ctx.cmd_bind_index_buffer(
                frame.command_buffer(),
                frame_draw.index_buffer.raw(),
                0,
                vk::IndexType::UINT32,
            );
            ctx.cmd_push_constants(
                frame.command_buffer(),
                self.pipeline_layout.raw,
                vk::ShaderStageFlags::VERTEX,
                0,
                &frame_draw
                    .vertex_buffer
                    .buffer_device_address()
                    .to_le_bytes(),
            );
            ctx.cmd_push_constants(
                frame.command_buffer(),
                self.pipeline_layout.raw,
                vk::ShaderStageFlags::VERTEX,
                8,
                &frame_draw.transforms.buffer_device_address().to_le_bytes(),
            );
        }

        let mut last_bound_pipeline = vk::Pipeline::null();
        for draw_params in frame_draw.draw_params.drain(0..) {
            // Bind the pipeline for the current draw, but only if its
            // actually different from the most recently used pipeline.
            let pipeline = draw_params.material.pipeline().raw;
            if pipeline != last_bound_pipeline {
                unsafe {
                    ctx.cmd_bind_pipeline(
                        frame.command_buffer(),
                        vk::PipelineBindPoint::GRAPHICS,
                        pipeline,
                    );
                }
                last_bound_pipeline = pipeline;
            }
            unsafe {
                ctx.cmd_push_constants(
                    frame.command_buffer(),
                    self.pipeline_layout.raw,
                    vk::ShaderStageFlags::VERTEX,
                    16,
                    &draw_params.transform_index.to_le_bytes(),
                );
                ctx.cmd_set_scissor(
                    frame.command_buffer(),
                    0,
                    &[draw_params.scissor],
                );
                ctx.cmd_draw_indexed(
                    frame.command_buffer(),
                    draw_params.index_count, // index count
                    1,                       // instance count
                    draw_params.index_offset, // first index
                    draw_params.vertex_offset as i32, // vertex offset
                    0,                       // first instance
                );
            }
        }

        Ok(())
    }
}
