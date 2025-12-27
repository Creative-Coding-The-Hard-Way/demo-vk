// mod egui_integration;
mod frame_metrics;
mod rolling_average;

use {
    self::frame_metrics::FrameMetrics,
    crate::{
        app::{app_main, App, AppState},
        graphics::vulkan::{
            Frame, FrameStatus, FramesInFlight, PresentImageStatus,
            RequiredDeviceFeatures, Swapchain, VulkanContext,
        },
        unwrap_here,
    },
    anyhow::Result,
    ash::vk::{self},
    clap::Parser,
    spin_sleep_util::Interval,
    std::{
        sync::Arc,
        time::{Duration, Instant},
    },
    winit::{
        dpi::PhysicalSize,
        event::{DeviceEvent, WindowEvent},
        keyboard::{KeyCode, PhysicalKey},
        window::Window,
    },
};

// pub use self::egui_integration::glfw_event_to_egui_event;

/// Standard graphics resources provided by the Demo.
pub struct Graphics {
    /// The Vulkan entrypoint
    pub vulkan: Arc<VulkanContext>,

    /// FramesInFlight synchronizes N frames in flight.
    pub frames_in_flight: FramesInFlight,

    /// The application's swapchain.
    pub swapchain: Swapchain,

    pub metrics: FrameMetrics,

    fps_limiter: Interval,

    swapchain_needs_rebuild: bool,
    paused: bool,
}

/// A demo is an opinionated application that automatically creates the
/// VulkanContext, Swapchain, FramesInFlight and other common utilities.
///
/// The demo splits the application's update() function into two parts:
/// * update(): update the application's state
/// * draw(): build the frame's command buffer
///
/// Draw is separate because update() does not wait for the next swapchain
/// image. There are some operations that may not depend on the swapchain and
/// can reasonably be started before the frame's command buffer is ready to be
/// recorded. (ticking a physics simulation, etc...)
pub trait Demo {
    type Args: Sized + Parser;
    const FRAMES_IN_FLIGHT_COUNT: usize = 3;
    const INITIAL_WINDOW_SIZE: (i32, i32) = (1024, 768);
    const FRAMES_PER_SECOND: u32 = 120;

    /// Creates a new instance of the demo.
    /// The application is allowed to modify the window based on its own
    /// requirements. This includes modifying the polling state, fullscreen
    /// status, size, etc...
    fn new(
        window: &mut Window,
        gfx: &mut Graphics,
        args: &Self::Args,
    ) -> Result<Self>
    where
        Self: Sized;

    /// Returns the required device features for this demo.
    fn required_device_features() -> RequiredDeviceFeatures {
        RequiredDeviceFeatures::default()
    }

    /// Handles a single window event.
    fn handle_window_event(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Graphics,
        #[allow(unused_variables)] event: WindowEvent,
    ) -> Result<AppState> {
        if let WindowEvent::KeyboardInput { event, .. } = event {
            if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                // If Esc is pressed, then quit
                return Ok(AppState::Exit);
            }
        }
        Ok(AppState::Continue)
    }

    /// Handles a single device event.
    fn handle_device_event(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Graphics,
        #[allow(unused_variables)] event: DeviceEvent,
    ) -> Result<AppState> {
        Ok(AppState::Continue)
    }

    /// The application is updated once every frame right before calling
    /// [Self::draw].
    fn update(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Graphics,
    ) -> Result<AppState> {
        Ok(AppState::Continue)
    }

    /// Build the command buffer for the next frame.
    ///
    /// Called after update() once the Frame is started.
    fn draw(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Graphics,
        #[allow(unused_variables)] frame: &Frame,
    ) -> Result<AppState> {
        // By default, this method does nothing but transition the swapchain
        // image to presentation format.
        //
        // Normally, a real application will use a render pass to do this.

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

        Ok(AppState::Continue)
    }

    /// Rebuild any of the demo's swapchain-dependent resources.
    ///
    /// All pending frames in flight are guaranteed to have completed by the
    /// time this function is called. E.G. any previously-recorded Frame command
    /// buffers are guaranteed to be finished.
    fn rebuild_swapchain_resources(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Graphics,
    ) -> Result<()> {
        Ok(())
    }

    /// Called when the application is paused.
    ///
    /// The application is paused automatically any time the framebuffer has
    /// size of 0. This occurs when the application is minimized, etc..
    fn paused(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Graphics,
    ) -> Result<()> {
        Ok(())
    }

    /// Called when the application is unpaused.
    fn unpaused(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Graphics,
    ) -> Result<()> {
        Ok(())
    }
}

struct DemoApp<D: Demo> {
    graphics: Graphics,
    demo: D,
}

impl<D: Demo> DemoApp<D> {
    /// Called any time the framebuffer size may have changed.
    /// Returns the current paused status for convenience.
    fn check_paused(&mut self, window: &mut Window) -> Result<bool> {
        let PhysicalSize {
            width: w,
            height: h,
        } = window.inner_size();
        let should_pause = w == 0 || h == 0;

        if should_pause {
            if !self.graphics.paused {
                self.demo.paused(window, &mut self.graphics)?;
            }
            self.graphics.paused = true;
        } else {
            if self.graphics.paused {
                self.demo.unpaused(window, &mut self.graphics)?;
            }
            self.graphics.paused = false;
            self.graphics.metrics.unpause();
        }
        Ok(self.graphics.paused)
    }
}

impl<D: Demo + Sized> App for DemoApp<D> {
    type Args = D::Args;

    fn new(window: &mut Window, args: &Self::Args) -> Result<Self>
    where
        Self: Sized,
    {
        let _ = window.request_inner_size(PhysicalSize {
            width: D::INITIAL_WINDOW_SIZE.0,
            height: D::INITIAL_WINDOW_SIZE.1,
        });
        window.set_title(std::any::type_name::<D>());

        let vulkan = unwrap_here!(
            "Create Vulkan context",
            VulkanContext::new(window, D::required_device_features())
        );

        let PhysicalSize {
            width: w,
            height: h,
        } = window.inner_size();
        let swapchain = unwrap_here!(
            "Create initial swapchain",
            Swapchain::new(vulkan.clone(), (w as u32, h as u32), None)
        );

        let frames_in_flight = unwrap_here!(
            "Create frames-in-flight",
            FramesInFlight::new(
                vulkan.clone(),
                swapchain.images().len(),
                D::FRAMES_IN_FLIGHT_COUNT,
            )
        );

        let fps_limiter = spin_sleep_util::interval(Duration::from_secs_f64(
            1.0 / D::FRAMES_PER_SECOND as f64,
        ));

        let mut graphics = Graphics {
            fps_limiter,
            vulkan,
            swapchain,
            frames_in_flight,
            metrics: FrameMetrics::new(D::FRAMES_PER_SECOND as usize),
            swapchain_needs_rebuild: false,
            paused: false,
        };
        let demo = unwrap_here!(
            "Initialize Demo",
            D::new(window, &mut graphics, args)
        );

        let mut app = Self { graphics, demo };
        unwrap_here!("Render first frame", app.update(window));

        // only show the window after rendering the first frame
        window.set_visible(true);

        Ok(app)
    }

    fn handle_window_event(
        &mut self,
        window: &mut Window,
        event: WindowEvent,
    ) -> Result<AppState> {
        self.demo
            .handle_window_event(window, &mut self.graphics, event)
    }

    fn handle_device_event(
        &mut self,
        window: &mut Window,
        event: DeviceEvent,
    ) -> Result<AppState> {
        self.demo
            .handle_device_event(window, &mut self.graphics, event)
    }

    fn update(&mut self, window: &mut Window) -> Result<AppState> {
        if self.graphics.paused && self.check_paused(window)? {
            std::hint::spin_loop();
            return Ok(AppState::Continue);
        }

        if self.graphics.swapchain_needs_rebuild {
            unwrap_here!(
                "Swapchain needs rebuild - wait for all frames to complete",
                self.graphics
                    .frames_in_flight
                    .wait_for_all_frames_to_complete()
            );
            if self.check_paused(window)? {
                return Ok(AppState::Continue);
            }
            let window_size = window.inner_size();
            self.graphics.swapchain = unwrap_here!(
                "Swapchain needs rebuild - rebuild swapchain",
                Swapchain::new(
                    self.graphics.vulkan.clone(),
                    (window_size.width as u32, window_size.height as u32),
                    Some(self.graphics.swapchain.raw()),
                )
            );

            unwrap_here!(
                "Rebuild Demo's swapchain resources",
                self.demo
                    .rebuild_swapchain_resources(window, &mut self.graphics)
            );

            self.graphics.swapchain_needs_rebuild = false;
        }

        self.graphics.metrics.frame_tick();

        // Update application logic
        // ------------------------

        let before_update = Instant::now();
        if unwrap_here!(
            "Demo::update()",
            self.demo.update(window, &mut self.graphics)
        ) == AppState::Exit
        {
            return Ok(AppState::Exit);
        }
        self.graphics.metrics.update_tick(before_update);

        // Limit FPS, wait just before acquiring the frame
        self.graphics.fps_limiter.tick();

        // Prepare frame command buffer and submit
        // ---------------------------------------

        let before_draw = Instant::now();
        let frame = match self
            .graphics
            .frames_in_flight
            .start_frame(&self.graphics.swapchain)?
        {
            FrameStatus::FrameStarted(frame) => frame,
            FrameStatus::SwapchainNeedsRebuild => {
                self.graphics.swapchain_needs_rebuild = true;
                return Ok(AppState::Continue);
            }
        };

        if unwrap_here!(
            "Demo draw",
            self.demo.draw(window, &mut self.graphics, &frame)
        ) == AppState::Exit
        {
            return Ok(AppState::Exit);
        }

        let result = self
            .graphics
            .frames_in_flight
            .present_frame(&self.graphics.swapchain, frame)?;
        if result == PresentImageStatus::SwapchainNeedsRebuild {
            self.graphics.swapchain_needs_rebuild = true;
        }
        self.graphics.metrics.draw_tick(before_draw);

        Ok(AppState::Continue)
    }
}

/// The main entrypoint for a demo.
pub fn demo_main<D: Demo + 'static>() -> Result<()> {
    app_main::<DemoApp<D>>()
}
