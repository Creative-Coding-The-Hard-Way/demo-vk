use {
    anyhow::Result,
    ash::vk,
    clap::Parser,
    demo_vk::{
        demo::{demo_main, Demo, Graphics},
        graphics::vulkan::Frame,
    },
    glfw::Window,
    std::time::Duration,
};

#[derive(Debug, Parser)]
struct Args {}

type Gfx = Graphics<Args>;

struct ExampleDemo {}

impl Demo for ExampleDemo {
    type Args = Args;
    const FRAMES_PER_SECOND: u32 = 5;

    fn new(_window: &mut Window, _gfx: &mut Gfx) -> Result<Self> {
        Ok(Self {})
    }

    fn draw(
        &mut self,
        _window: &mut Window,
        gfx: &mut Gfx,
        frame: &Frame,
    ) -> Result<()> {
        std::thread::sleep(Duration::from_millis(10));

        let image_memory_barrier = vk::ImageMemoryBarrier {
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::PRESENT_SRC_KHR,
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
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        Ok(())
    }
}

fn main() {
    demo_main::<ExampleDemo>();
}
