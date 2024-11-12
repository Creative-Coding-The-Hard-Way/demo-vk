mod descriptors;
mod frame_resources;
mod pipeline;
mod sprite;
mod sprite_batch;

use {
    self::frame_resources::FrameResources,
    super::vulkan::Frame,
    crate::graphics::vulkan::{
        raii, DescriptorBumpAllocator, FramesInFlight, UniformBuffer,
        VulkanContext,
    },
    anyhow::Result,
    ash::vk,
    bon::bon,
    nalgebra::Matrix4,
    std::sync::Arc,
};

pub use self::{
    sprite::Sprite,
    sprite_batch::{SpriteBatch, StreamingSprites},
};

/// Layer data provided to the graphics pipeline.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct LayerData<UserDataT: Copy + Clone + Default> {
    pub projection: [[f32; 4]; 4],
    pub user_data: UserDataT,
}

pub struct SpriteLayerCommands<'a, 'b, UserDataT>(
    &'a mut SpriteLayer<UserDataT>,
    &'b Frame,
)
where
    UserDataT: Copy + Clone + Default;

impl<UserDataT> SpriteLayerCommands<'_, '_, UserDataT>
where
    UserDataT: Copy + Clone + Default,
{
    /// Draw a batch of sprites.
    pub fn draw(self, batch: &impl SpriteBatch) -> Result<Self> {
        let Self(layer, frame) = self;
        layer.draw_batch(frame, batch)?;
        Ok(Self(layer, frame))
    }

    /// Finishes rendering the sprite layer on the frame.
    pub fn finish(self) {
        // no-op
    }
}

/// A sprite layer contains all of the resources to render batches of Sprites.
///
/// Multiple layers can be used for batches of sprites with different
/// perspective transforms. (a scene layer and a ui layer, for example).
/// Additionally, layers can be used to employ unique sprite shaders.
pub struct SpriteLayer<UserDataT: Copy + Clone + Default> {
    layer_descriptor_set_layout: raii::DescriptorSetLayout,
    batch_descriptor_set_layout: raii::DescriptorSetLayout,

    frames: Vec<FrameResources>,

    descriptor_allocator: DescriptorBumpAllocator,
    fragment_shader: Arc<raii::ShaderModule>,
    pipeline_layout: raii::PipelineLayout,
    pipeline: raii::Pipeline,
    layer_data_buffer: UniformBuffer<LayerData<UserDataT>>,
    current_layer_data: LayerData<UserDataT>,
    ctx: Arc<VulkanContext>,
}

#[bon]
impl<UserDataT: Copy + Clone + Default> SpriteLayer<UserDataT> {
    /// Creates a new sprite layer for use with the provided render pass.
    #[builder]
    pub fn new(
        ctx: Arc<VulkanContext>,
        frames_in_flight: &FramesInFlight,
        render_pass: &raii::RenderPass,
        projection: Matrix4<f32>,
        texture_atlas_layout: &raii::DescriptorSetLayout,
        fragment_shader: Option<Arc<raii::ShaderModule>>,
    ) -> Result<Self> {
        let layer_descriptor_set_layout =
            descriptors::create_layer_descriptor_set_layout(&ctx)?;
        let batch_descriptor_set_layout =
            descriptors::create_batch_descriptor_set_layout(&ctx)?;
        let mut descriptor_allocator =
            descriptors::create_descriptor_allocator(ctx.clone())?;

        let default_fragment_shader = pipeline::default_fragment_shader(&ctx)?;
        let fragment_shader =
            fragment_shader.unwrap_or(default_fragment_shader);

        let pipeline_layout = pipeline::create_pipeline_layout(
            &ctx,
            &[
                texture_atlas_layout,
                &layer_descriptor_set_layout,
                &batch_descriptor_set_layout,
            ],
        )?;
        let pipeline = pipeline::create_pipeline(
            &ctx,
            &pipeline_layout,
            render_pass,
            &fragment_shader,
        )?;

        let layer_data_buffer =
            UniformBuffer::allocate_per_frame(&ctx, frames_in_flight)?;

        let current_layer_data = LayerData::<UserDataT> {
            projection: projection.data.0,
            user_data: UserDataT::default(),
        };

        let mut frames = vec![];
        for index in 0..frames_in_flight.frame_count() {
            frames.push(FrameResources::new(
                ctx.clone(),
                &mut descriptor_allocator,
                &layer_descriptor_set_layout,
                &layer_data_buffer,
                index,
            )?);
        }

        Ok(Self {
            fragment_shader,
            layer_descriptor_set_layout,
            batch_descriptor_set_layout,
            frames,
            descriptor_allocator,
            pipeline_layout,
            pipeline,
            layer_data_buffer,
            current_layer_data,
            ctx,
        })
    }

    /// Begin rendering a layer to the frame.
    pub fn begin_frame_commands<'a, 'b>(
        &'a mut self,
        frame: &'b Frame,
    ) -> Result<SpriteLayerCommands<'a, 'b, UserDataT>> {
        self.bind_pipeline(frame)?;
        Ok(SpriteLayerCommands(self, frame))
    }

    /// Reset the Sprite Layer's internal resources.
    ///
    /// This can be more efficient than destroying and recreating a new sprite
    /// layer.
    ///
    /// # Performance
    ///
    /// This method waits for all pending frames in flight to complete.
    pub fn reset(&mut self, frames_in_flight: &FramesInFlight) -> Result<()> {
        frames_in_flight.wait_for_all_frames_to_complete()?;
        self.frames.clear();
        unsafe {
            // SAFE: because there are no pending frames-in-flight
            self.descriptor_allocator.reset()?;
        }

        for index in 0..frames_in_flight.frame_count() {
            self.frames.push(FrameResources::new(
                self.ctx.clone(),
                &mut self.descriptor_allocator,
                &self.layer_descriptor_set_layout,
                &self.layer_data_buffer,
                index,
            )?);
        }

        Ok(())
    }

    /// Rebuild the graphics pipeline
    ///
    /// # Safety
    ///
    /// Unsafe because this should only be called when the layer is not being
    /// used by the GPU. For example, after a device wait_idle or after all
    /// frames in flight have completed.
    pub unsafe fn rebuild_pipeline(
        &mut self,
        renderpass: &raii::RenderPass,
        fragment_shader: Option<Arc<raii::ShaderModule>>,
    ) -> Result<()> {
        if let Some(shader) = fragment_shader {
            self.fragment_shader = shader;
        }
        self.pipeline = pipeline::create_pipeline(
            &self.ctx,
            &self.pipeline_layout,
            renderpass,
            &self.fragment_shader,
        )?;
        Ok(())
    }

    /// Sets the layer's projection matrix. This will take effect in the next
    /// [Self::begin_frame_commands] call.
    pub fn set_projection(&mut self, projection: &Matrix4<f32>) {
        self.current_layer_data.projection = projection.data.0;
    }

    /// Sets the layer's user data for the frame. This will take effect in the
    /// next [Self::begin_frame_commands] call.
    pub fn set_user_data(&mut self, user_data: UserDataT) {
        self.current_layer_data.user_data = user_data;
    }

    // Private API ------------------------------------------------------------

    /// Binds the pipeline to the frame command buffer.
    fn bind_pipeline(&mut self, frame: &Frame) -> Result<()> {
        self.layer_data_buffer
            .update_frame_data(frame, self.current_layer_data)?;

        let resources = &mut self.frames[frame.frame_index()];
        unsafe {
            self.ctx.cmd_bind_pipeline(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.raw,
            );
            self.ctx.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                1,
                &[resources.get_layer_descriptor()],
                &[],
            );
        }
        Ok(())
    }

    /// Binds a descriptor set for the batch and adds a draw command to the
    /// frame's command buffer. Note: it is only valid to call this method after
    /// a corresponding call to [Self::bind_pipeline].
    fn draw_batch(
        &mut self,
        frame: &Frame,
        batch: &impl SpriteBatch,
    ) -> Result<()> {
        if batch.count() == 0 {
            // no-op for empty batches
            return Ok(());
        }

        let resources = &mut self.frames[frame.frame_index()];
        let batch_descriptor = resources.get_batch_descriptor(
            batch,
            &mut self.descriptor_allocator,
            &self.batch_descriptor_set_layout,
        )?;
        unsafe {
            self.ctx.cmd_set_viewport(
                frame.command_buffer(),
                0,
                &[batch.viewport()],
            );
            self.ctx.cmd_set_scissor(
                frame.command_buffer(),
                0,
                &[batch.scissor()],
            );
            self.ctx.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                2,
                &[batch_descriptor],
                &[],
            );
            self.ctx
                .cmd_draw(frame.command_buffer(), 6, batch.count(), 0, 0);
        }
        Ok(())
    }
}
