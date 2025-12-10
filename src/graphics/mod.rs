pub mod streaming_renderer;
pub mod vulkan;

use {
    crate::graphics::vulkan::VulkanContext, ash::vk, bon::builder,
    nalgebra::Matrix4,
};

pub fn ortho_projection(aspect: f32, height: f32) -> Matrix4<f32> {
    let w = height * aspect;
    let h = height;
    #[rustfmt::skip]
    let projection = Matrix4::new(
        2.0 / w,  0.0,     0.0, 0.0,
        0.0,     -2.0 / h, 0.0, 0.0,
        0.0,      0.0,     1.0, 0.0,
        0.0,      0.0,     0.0, 1.0,
    );
    projection
}

/// Write a pipeline barrier to the command buffer to transition the image
/// layout.
#[builder]
pub fn image_memory_barrier(
    ctx: &VulkanContext,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
    src_access_mask: vk::AccessFlags,
    dst_access_mask: vk::AccessFlags,
    #[builder(default = vk::PipelineStageFlags::ALL_COMMANDS)]
    src_stage_mask: vk::PipelineStageFlags,
    #[builder(default = vk::PipelineStageFlags::ALL_COMMANDS)]
    dst_stage_mask: vk::PipelineStageFlags,
) {
    let is_depth = dst_access_mask
        .contains(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ)
        || dst_access_mask
            .contains(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
        || src_access_mask
            .contains(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ)
        || src_access_mask
            .contains(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE);
    let image_memory_barrier = vk::ImageMemoryBarrier {
        old_layout,
        new_layout,
        src_access_mask,
        dst_access_mask,
        image,
        subresource_range: vk::ImageSubresourceRange {
            aspect_mask: if is_depth {
                vk::ImageAspectFlags::DEPTH
            } else {
                vk::ImageAspectFlags::COLOR
            },
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        },
        ..Default::default()
    };
    unsafe {
        ctx.cmd_pipeline_barrier(
            command_buffer,
            src_stage_mask,
            dst_stage_mask,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[image_memory_barrier],
        );
    }
}
