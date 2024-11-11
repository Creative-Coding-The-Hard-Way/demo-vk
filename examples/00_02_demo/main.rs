use {
    anyhow::Result,
    clap::Parser,
    demo_vk::{
        demo::{demo_main, Demo, Graphics},
        graphics::{vulkan::Frame, SwapchainColorPass},
    },
    glfw::Window,
    std::time::Instant,
};

#[derive(Debug, Parser)]
struct Args {}

type Gfx = Graphics<Args>;

struct ExampleDemo {
    color_pass: SwapchainColorPass,
}

impl Demo for ExampleDemo {
    type Args = Args;

    fn new(_window: &mut Window, gfx: &mut Gfx) -> Result<Self> {
        let color_pass =
            SwapchainColorPass::new(gfx.vulkan.clone(), &gfx.swapchain)?;
        Ok(Self { color_pass })
    }

    fn draw(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Gfx,
        #[allow(unused_variables)] frame: &Frame,
    ) -> Result<()> {
        log::info!("{}", gfx.metrics);

        let before_color_pass = Instant::now();
        self.color_pass
            .begin_render_pass(frame, [0.2, 0.2, 0.25, 1.0]);
        self.color_pass.end_render_pass(frame);
        gfx.metrics.ms_since("color pass ms", before_color_pass);

        Ok(())
    }

    fn rebuild_swapchain_resources(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Gfx,
    ) -> Result<()> {
        self.color_pass =
            SwapchainColorPass::new(gfx.vulkan.clone(), &gfx.swapchain)?;
        Ok(())
    }
}

fn main() {
    demo_main::<ExampleDemo>();
}
