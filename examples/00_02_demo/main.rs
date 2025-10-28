use {
    anyhow::Result,
    clap::Parser,
    demo_vk::{
        demo::{demo_main, Demo, Graphics},
        graphics::{vulkan::Frame, SwapchainColorPass},
    },
    glfw::Window,
    std::time::{Duration, Instant},
};

#[derive(Debug, Parser)]
struct Args {}

type Gfx = Graphics<Args>;

struct ExampleDemo {
    tick: Instant,
    color_pass: SwapchainColorPass,
}

impl Demo for ExampleDemo {
    type Args = Args;

    fn new(_window: &mut Window, gfx: &mut Gfx) -> Result<Self> {
        let color_pass =
            SwapchainColorPass::new(gfx.vulkan.clone(), &gfx.swapchain)?;
        Ok(Self {
            tick: Instant::now(),
            color_pass,
        })
    }

    fn draw(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Gfx,
        #[allow(unused_variables)] frame: &Frame,
    ) -> Result<()> {
        let start = Instant::now();
        if start.duration_since(self.tick) >= Duration::from_secs(1) {
            self.tick = start;
            log::info!("{}", gfx.metrics);
        }

        std::thread::sleep(Duration::from_millis(10));

        self.color_pass
            .begin_render_pass(frame, [0.0, 0.0, 0.0, 0.0]);
        self.color_pass.end_render_pass(frame);
        gfx.metrics.ms_since("color pass ms", start);

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
