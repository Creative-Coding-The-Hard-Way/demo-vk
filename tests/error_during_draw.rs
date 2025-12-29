//! This test exercises a failure discovered in the frames_in_flight
//! implementation where ar error raised during the demo's draw() method would
//! cause the application to hang, rather than exit with a message.

use {
    anyhow::{bail, Result},
    clap::Parser,
    demo_vk::{
        app::AppState,
        demo::{demo_main, Demo, Graphics},
        graphics::vulkan::Frame,
    },
    winit::window::Window,
};

#[derive(Debug, Parser)]
struct Args {}

struct ErrorDuringDraw;

impl Demo for ErrorDuringDraw {
    type Args = Args;

    fn new(
        _window: &mut Window,
        _gfx: &mut Graphics,
        _args: &Self::Args,
    ) -> Result<Self> {
        Ok(Self {})
    }

    fn draw(
        &mut self,
        _window: &mut Window,
        _gfx: &mut Graphics,
        _frame: &Frame,
    ) -> Result<AppState> {
        bail!("Error after acquiring a frame!");
    }
}

fn main() {
    let result = demo_main::<ErrorDuringDraw>();
    assert!(result.is_err());
    // unable to initialize because the first frame is rendered during
    // setup
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("Unable to initialize app"));
}
