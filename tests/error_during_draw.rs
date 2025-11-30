//! This test exercises a failure discovered in the frames_in_flight
//! implementation where ar error raised during the demo's draw() method would
//! cause the application to hang, rather than exit with a message.

use {
    anyhow::{bail, Result},
    clap::Parser,
    demo_vk::{
        demo::{demo_main, Demo, Graphics},
        graphics::vulkan::Frame,
    },
    glfw::Window,
};

#[derive(Debug, Parser)]
struct Args {}

struct ErrorDuringDraw;

impl Demo for ErrorDuringDraw {
    type Args = Args;

    fn new(_window: &mut Window, _gfx: &mut Graphics<Args>) -> Result<Self> {
        Ok(Self {})
    }

    fn draw(
        &mut self,
        _window: &mut Window,
        _gfx: &mut Graphics<Args>,
        _frame: &Frame,
    ) -> Result<()> {
        bail!("Error after acquiring a frame!");
    }
}

#[test]
fn when_an_error_is_raised_in_draw_then_the_application_should_exit() {
    let result = demo_main::<ErrorDuringDraw>();
    assert!(result.is_err());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("Unhandled error in Demo::draw()"));
}
