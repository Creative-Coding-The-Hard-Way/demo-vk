use {
    crate::graphics::vulkan::{raii, VulkanContext},
    anyhow::{Context, Result},
    ash::vk,
    std::ffi::CStr,
};

/// The shader entrypoint name, always defaults to 'main'.
const SHADER_ENTRYPOINT: &CStr = c"main";

/// Materials are used to style mesh properties.
///
/// Materials are immutable and can be shared by meshes.
///
/// # Vertex Shader
///
/// ```glsl
/// #version 460
/// #pragma shader_stage(vertex)
///
/// // Adds support for buffer references, used for vertex data
/// #extension GL_EXT_buffer_reference : enable
///
/// // Adds support for non-uniform indexing and variable sized descriptor arrays
/// #extension GL_EXT_nonuniform_qualifier : require
///
/// #extension GL_EXT_shader_explicit_arithmetic_types_int32: enable
///
/// struct Vertex {
///     vec3 pos;
///     float uv_x;
///     vec4 color;
///     int texture_index;
///     float uv_y;
/// };
///
/// struct MeshTransform {
///     mat4 transform;
/// };
///
/// // textures bound to set 0
/// layout(set = 0, binding = 0) uniform sampler u_Sampler;
/// layout(set = 0, binding = 1) uniform texture2D u_Textures[];
///
/// // Optional Frame Data
/// // layout(set = 1, binding = 0) uniform ubo {
/// //     float delta_time;
/// // } u_FrameConstants;
///
/// // Push Constants
/// layout(buffer_reference, std430) readonly buffer VertexBuffer {
///     Vertex data[];
/// };
/// layout(buffer_reference, std430) readonly buffer TransformBuffer {
///     MeshTransform data[];
/// };
/// layout(push_constant) uniform constants {
///     VertexBuffer vertices;
///     TransformBuffer mesh_transforms;
///     uint32_t transform_index;
/// } pc_Constants;
///
/// // Per-Vertex outputs
/// layout(location = 0) out vec4 out_VertexColor;
/// layout(location = 1) out vec2 out_UV;
/// layout(location = 2) flat out int out_TextureIndex;
///
/// void main() {
///     Vertex vert = pc_Constants.vertices.data[gl_VertexIndex];
///     out_VertexColor = vert.color;
///     out_TextureIndex = vert.texture_index;
///     out_UV = vec2(vert.uv_x, vert.uv_y);
///
///     mat4 transform =
///         pc_Constants.mesh_transforms.data[pc_Constants.transform_index].transform;
///
///     gl_Position = transform * vec4(vert.pos.x, vert.pos.y, 0.0, 1.0);
/// }
/// ```
///
/// # Fragment Shader
///
/// ```glsl
/// #version 460
/// #pragma shader_stage(fragment)
///
/// // Adds support for non-uniform indexing and variable sized descriptor arrays
/// #extension GL_EXT_nonuniform_qualifier : require
///
/// // textures bound to set 0
/// layout(set = 0, binding = 0) uniform sampler u_Sampler;
/// layout(set = 0, binding = 1) uniform texture2D u_Textures[];
///
/// // Inputs
/// layout(location = 0) in vec4 in_VertexColor;
/// layout(location = 1) in vec2 in_UV;
/// layout(location = 2) flat in int in_TextureIndex;
///
/// // Outputs
/// layout(location = 0) out vec4 out_FragColor;
///
/// void main() {
///     vec4 tex_color = vec4(1.0);
///
///     // Only perform texture fetch if a texture is specified.
///     if (in_TextureIndex >= 0) {
///         tex_color *= texture(
///                 nonuniformEXT(sampler2D(u_Textures[in_TextureIndex], u_Sampler)),
///                 in_UV
///             );
///     }
///
///     out_FragColor = in_VertexColor * tex_color;
/// }
/// ```
#[derive(Debug)]
pub struct Material {
    pipeline: raii::Pipeline,
}

impl Material {
    /// Builds a pipeline layout for use with Material pipelines.
    pub(super) fn create_pipeline_layout(
        ctx: &VulkanContext,
        texture_atlas_descriptor_set_layout: &raii::DescriptorSetLayout,
        frame_constants_descriptor_set_layout: &raii::DescriptorSetLayout,
    ) -> Result<raii::PipelineLayout> {
        let raw_descriptor_set_layouts = [
            texture_atlas_descriptor_set_layout.raw,
            frame_constants_descriptor_set_layout.raw,
        ];
        let push_constant_ranges = [vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::VERTEX,
            offset: 0,
            size: 8 + 8 + 4,
        }];
        raii::PipelineLayout::new(
            "FirstTriangle",
            ctx.device.clone(),
            &vk::PipelineLayoutCreateInfo {
                set_layout_count: raw_descriptor_set_layouts.len() as u32,
                p_set_layouts: raw_descriptor_set_layouts.as_ptr(),
                push_constant_range_count: push_constant_ranges.len() as u32,
                p_push_constant_ranges: push_constant_ranges.as_ptr(),
                ..Default::default()
            },
        )
    }

    /// Creates a new material for use when rendering meshes.
    pub(super) fn new(
        ctx: &VulkanContext,
        image_format: vk::Format,
        pipeline_layout: &raii::PipelineLayout,
        vertex_shader_module: &raii::ShaderModule,
        fragment_shader_module: &raii::ShaderModule,
    ) -> Result<Self> {
        let stages = [
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::VERTEX,
                module: vertex_shader_module.raw,
                p_name: SHADER_ENTRYPOINT.as_ptr(),
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: fragment_shader_module.raw,
                p_name: SHADER_ENTRYPOINT.as_ptr(),
                ..Default::default()
            },
        ];

        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
            vertex_binding_description_count: 0,
            vertex_attribute_description_count: 0,
            ..Default::default()
        };
        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: vk::FALSE,
            ..Default::default()
        };
        let tesselation_state = vk::PipelineTessellationStateCreateInfo {
            patch_control_points: 0,
            ..Default::default()
        };
        let rasterization_state = vk::PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: vk::FALSE,
            rasterizer_discard_enable: vk::FALSE,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::NONE,
            front_face: vk::FrontFace::CLOCKWISE,
            depth_bias_enable: vk::FALSE,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
            ..Default::default()
        };
        let multisample_state = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: vk::FALSE,
            min_sample_shading: 1.0,
            p_sample_mask: std::ptr::null(),
            alpha_to_coverage_enable: vk::FALSE,
            alpha_to_one_enable: vk::FALSE,
            ..Default::default()
        };
        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo {
            depth_test_enable: vk::TRUE,
            depth_write_enable: vk::TRUE,
            depth_compare_op: vk::CompareOp::LESS,
            depth_bounds_test_enable: vk::FALSE,
            stencil_test_enable: vk::FALSE,
            min_depth_bounds: 0.0,
            max_depth_bounds: 1.0,
            ..Default::default()
        };
        let color_blend_statetachment_state =
            vk::PipelineColorBlendAttachmentState {
                blend_enable: vk::TRUE,
                src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
                dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
                color_blend_op: vk::BlendOp::ADD,
                src_alpha_blend_factor: vk::BlendFactor::ONE,
                dst_alpha_blend_factor: vk::BlendFactor::ZERO,
                alpha_blend_op: vk::BlendOp::ADD,
                color_write_mask: vk::ColorComponentFlags::RGBA,
            };
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
            logic_op_enable: vk::FALSE,
            logic_op: vk::LogicOp::COPY,
            attachment_count: 1,
            p_attachments: &color_blend_statetachment_state,
            blend_constants: [0.0, 0.0, 0.0, 0.0],
            ..Default::default()
        };

        let dynamic_states =
            [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state = vk::PipelineDynamicStateCreateInfo {
            dynamic_state_count: dynamic_states.len() as u32,
            p_dynamic_states: dynamic_states.as_ptr(),
            ..Default::default()
        };

        let viewport_state = vk::PipelineViewportStateCreateInfo {
            viewport_count: 1,
            scissor_count: 1,
            ..Default::default()
        };

        let color_attachment_formats = [image_format];
        let mut rendering_info = vk::PipelineRenderingCreateInfo {
            view_mask: 0,
            color_attachment_count: 1,
            p_color_attachment_formats: color_attachment_formats.as_ptr(),
            depth_attachment_format: vk::Format::D32_SFLOAT,
            ..Default::default()
        };

        let create_info = vk::GraphicsPipelineCreateInfo {
            stage_count: stages.len() as u32,
            p_stages: stages.as_ptr(),
            p_vertex_input_state: &vertex_input_state,
            p_input_assembly_state: &input_assembly_state,
            p_tessellation_state: &tesselation_state,
            p_viewport_state: &viewport_state,
            p_rasterization_state: &rasterization_state,
            p_multisample_state: &multisample_state,
            p_depth_stencil_state: &depth_stencil_state,
            p_color_blend_state: &color_blend_state,
            p_dynamic_state: &dynamic_state,
            layout: pipeline_layout.raw,
            render_pass: vk::RenderPass::null(),
            subpass: 0,
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: 0,
            ..Default::default()
        }
        .push_next(&mut rendering_info);

        let pipeline = raii::Pipeline::new_graphics_pipeline(
            ctx.device.clone(),
            &create_info,
        )
        .context("Unable to create pipeline!")?;

        Ok(Self { pipeline })
    }

    /// Returns the pipeline used by this material.
    pub fn pipeline(&self) -> &raii::Pipeline {
        &self.pipeline
    }
}
