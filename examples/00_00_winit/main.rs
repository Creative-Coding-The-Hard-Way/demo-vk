use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

#[derive(Default)]
struct WinitApp {
    window: Option<Window>,
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("Application Resumed");

        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("Unable to create window!");
        let _ = window.request_inner_size(PhysicalSize::new(1024, 768));
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // draw frame
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {
                // no-op for now
            }
        }
    }
}

fn main() {
    println!("Hello World");
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = WinitApp::default();
    event_loop.run_app(&mut app).expect("No errors");
}
