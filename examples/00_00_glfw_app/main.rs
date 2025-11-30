use {
    anyhow::Result,
    clap::Parser,
    demo_vk::app::{app_main, App},
};

/// All apps accept arguments from the CLI via a type that implements the clap
/// Parser.
#[derive(Debug, Parser)]
struct Args {}

/// An app is just a struct. It can contain state and clean itself up in drop if
/// needed.
struct GLFWApp;

impl App for GLFWApp {
    type Args = Args;

    /// Create a new instance of the app.
    /// The glfw window is mutable and can be modified to suit the application's
    /// needs.
    fn new(window: &mut glfw::Window, _args: Self::Args) -> Result<Self> {
        window.set_all_polling(true);
        window.set_title(std::any::type_name::<Self>());
        Ok(Self {})
    }

    /// Handle a GLFW event.
    ///
    /// This is called every frame in a loop to process all events before the
    /// next update().
    fn handle_event(
        &mut self,
        window: &mut glfw::Window,
        event: glfw::WindowEvent,
    ) -> Result<()> {
        // Pattern match the event and tell the window to close if any key is
        // pressed.
        if let glfw::WindowEvent::Key(_, _, glfw::Action::Release, _) = event {
            window.set_should_close(true);
        }
        Ok(())
    }

    /// Update the application.
    ///
    /// This is called once a frame after all events are processed.
    fn update(&mut self, _window: &mut glfw::Window) -> Result<()> {
        Ok(())
    }
}

pub fn main() {
    // app_main creates an instance of the app and starts the GLFW loop to
    // process events, etc...
    let _ = app_main::<GLFWApp>();
}
