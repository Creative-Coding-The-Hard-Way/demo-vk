use {
    crate::pipeline::create_pipeline,
    anyhow::Result,
    ash::vk,
    clap::Parser,
    demo_vk::{
        app::AppState,
        demo::{demo_main, Demo, Graphics},
        graphics::vulkan::{raii, Frame, RequiredDeviceFeatures},
    },
    winit::window::Window,
};

mod pipeline;

#[derive(Debug, Parser)]
struct Args {}

struct FirstTriangle {
    pipeline: raii::Pipeline,
}

impl Demo for FirstTriangle {
    type Args = Args;

    fn required_device_features() -> RequiredDeviceFeatures {
        RequiredDeviceFeatures {
            physical_device_dynamic_rendering_features:
                vk::PhysicalDeviceDynamicRenderingFeatures {
                    dynamic_rendering: vk::TRUE,
                    ..Default::default()
                },
            ..Default::default()
        }
    }

    /// Initialize the demo
    fn new(
        _window: &mut Window,
        gfx: &mut Graphics,
        _args: &Args,
    ) -> Result<Self> {
        Ok(Self {
            pipeline: create_pipeline(gfx)?,
        })
    }

    /// Draw a frame
    fn draw(
        &mut self,
        _window: &mut Window,
        gfx: &mut Graphics,
        frame: &Frame,
    ) -> Result<AppState> {
        let image_memory_barrier = vk::ImageMemoryBarrier {
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            image: frame.swapchain_image(),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };
        unsafe {
            gfx.vulkan.cmd_pipeline_barrier(
                frame.command_buffer(),
                vk::PipelineStageFlags::TOP_OF_PIPE
                    | vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        unsafe {
            let color_attachments = [vk::RenderingAttachmentInfo {
                image_view: frame.swapchain_image_view(),
                image_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                resolve_mode: vk::ResolveModeFlags::NONE,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                },
                ..Default::default()
            }];
            gfx.vulkan.cmd_begin_rendering(
                frame.command_buffer(),
                &vk::RenderingInfo {
                    render_area: vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: gfx.swapchain.extent(),
                    },
                    layer_count: 1,
                    color_attachment_count: 1,
                    p_color_attachments: color_attachments.as_ptr(),
                    ..Default::default()
                },
            );

            gfx.vulkan.cmd_set_viewport(
                frame.command_buffer(),
                0,
                &[vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: gfx.swapchain.extent().width as f32,
                    height: gfx.swapchain.extent().height as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                }],
            );
            gfx.vulkan.cmd_set_scissor(
                frame.command_buffer(),
                0,
                &[vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: gfx.swapchain.extent(),
                }],
            );
            gfx.vulkan.cmd_bind_pipeline(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.raw,
            );
            gfx.vulkan.cmd_draw(frame.command_buffer(), 3, 1, 0, 0);

            gfx.vulkan.cmd_end_rendering(frame.command_buffer());
        }

        let image_memory_barrier = vk::ImageMemoryBarrier {
            old_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            new_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            src_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_access_mask: vk::AccessFlags::empty(),
            image: frame.swapchain_image(),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };
        unsafe {
            gfx.vulkan.cmd_pipeline_barrier(
                frame.command_buffer(),
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        Ok(AppState::Continue)
    }
}

fn main() {
    let _ = demo_main::<FirstTriangle>();
}
