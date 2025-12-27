//! The Winit ApplicationHandler implementation.
//!
//! This module defines the traits and functions required for managing the
//! lifecycle of a Winit application with a single Vulkan-enabled window.

mod logging;

use {
    crate::unwrap_here,
    anyhow::{Context, Result},
    clap::Parser,
    winit::{
        application::ApplicationHandler,
        event::{DeviceEvent, WindowEvent},
        event_loop::{ActiveEventLoop, EventLoop},
        window::{Window, WindowAttributes},
    },
};

pub use self::logging::{ErrorLocationMessage, Location};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AppState {
    Continue,
    Exit,
}

/// Implementations of this trait can be run with app_main to manage a Winit
/// window.
pub trait App {
    type Args: Sized + Parser;

    /// Creates a new instance of the application.
    /// The application is allowed to modify the window based on its own
    /// requirements. This includes modifying the polling state, fullscreen
    /// status, size, etc...
    ///
    /// Note: the window is not visible when initially created, the app must
    /// choose when to make it visible for the first time.
    fn new(window: &mut Window, args: &Self::Args) -> Result<Self>
    where
        Self: Sized;

    /// Handles a single WindowEvent.
    fn handle_window_event(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] event: WindowEvent,
    ) -> Result<AppState> {
        Ok(AppState::Continue)
    }

    /// Handles a single DeviceEvent.
    fn handle_device_event(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] event: DeviceEvent,
    ) -> Result<AppState> {
        Ok(AppState::Continue)
    }

    /// Called in a loop when no other events are pending or when the OS
    /// requests a new frame for the window.
    ///
    /// This is a good place for rendering logic. This method blocks event
    /// processing, so it should be kept as responsive as possible.
    fn update(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
    ) -> Result<AppState> {
        Ok(AppState::Continue)
    }
}

/// The entrypoint for implementations of the App trait.
///
/// Initializes logging and the application event loop. Any errors that cause
/// the application to exit are reported with a stacktrace if available.
pub fn app_main<A>() -> Result<()>
where
    A: App + 'static,
{
    let exit_result = try_app_main::<A>();
    if let Some(err) = exit_result.as_ref().err() {
        let result: String = err
            .chain()
            .skip(1)
            .enumerate()
            .map(|(index, err)| format!("  {}| {}\n\n", index, err))
            .to_owned()
            .collect();
        log::error!(
            "{}\n\n{}\n\nCaused by:\n{}\n\nBacktrace:\n{}",
            "Application exited with an error!",
            err,
            result,
            err.backtrace()
        );
    };
    exit_result
}

struct WinitAppHandler<A: App + 'static> {
    args: Option<A::Args>,
    app: Option<A>,
    window: Option<Window>,
    exit_result: Result<()>,
}

impl<A: App + 'static> WinitAppHandler<A> {
    fn process_app_state(
        &mut self,
        app_state: Result<AppState>,
        event_loop: &ActiveEventLoop,
    ) {
        if let Ok(AppState::Exit) = app_state {
            event_loop.exit();
        }
        if let Err(err) = app_state {
            self.exit_result =
                Err(err).context("Unexpected application error!");
            event_loop.exit();
        }
    }
}

impl<A: App + 'static> ApplicationHandler for WinitAppHandler<A> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_some() {
            // All setup logic is one-time only, so if the window already
            // exists then do nothing.
            return;
        }

        let mut window = event_loop
            .create_window(WindowAttributes::default().with_visible(false));

        if let Err(error) = window {
            self.exit_result = Err(error).context("Unable to create window");
            event_loop.exit();
            return;
        }

        let app = A::new(window.as_mut().unwrap(), &self.args.take().unwrap());
        if let Err(error) = app {
            self.exit_result = Err(error);
            event_loop.exit();
            return;
        }

        self.window = Some(window.unwrap());
        self.app = Some(app.unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let app_state = match event {
            WindowEvent::CloseRequested => Ok(AppState::Exit),
            WindowEvent::RedrawRequested => {
                self.window.as_mut().unwrap().pre_present_notify();
                let state = self
                    .app
                    .as_mut()
                    .unwrap()
                    .update(self.window.as_mut().unwrap())
                    .context("Unexpected error in App::update()!");
                self.window.as_mut().unwrap().request_redraw();
                state
            }
            _ => self
                .app
                .as_mut()
                .unwrap()
                .handle_window_event(self.window.as_mut().unwrap(), event),
        };
        self.process_app_state(app_state, event_loop);
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        let app_state = self
            .app
            .as_mut()
            .unwrap()
            .handle_device_event(self.window.as_mut().unwrap(), event);
        self.process_app_state(app_state, event_loop);
    }
}

fn try_app_main<A>() -> Result<()>
where
    A: App + 'static,
{
    logging::setup();

    let args = unwrap_here!(
        "Parse CLI args",
        argfile::expand_args(argfile::parse_fromfile, argfile::PREFIX)
    );

    let mut winit_app = WinitAppHandler::<A> {
        args: Some(A::Args::parse_from(args)),
        app: None,
        window: None,
        exit_result: Ok(()),
    };

    let event_loop = unwrap_here!("Create event loop.", EventLoop::new());
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    unwrap_here!(
        "Run main application loop.",
        event_loop.run_app(&mut winit_app)
    );

    winit_app.exit_result
}
