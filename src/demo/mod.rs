mod frame_metrics;
mod rolling_average;

use {
    self::frame_metrics::FrameMetrics,
    crate::{
        app::{app_main, App},
        graphics::vulkan::{
            Frame, FrameStatus, FramesInFlight, PresentImageStatus,
            RequiredDeviceFeatures, Swapchain, VulkanContext,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    clap::Parser,
    spin_sleep_util::Interval,
    std::{
        sync::Arc,
        time::{Duration, Instant},
    },
};

/// Standard graphics resources provided by the Demo.
pub struct Graphics<A> {
    /// The Vulkan entrypoint
    pub vulkan: Arc<VulkanContext>,

    /// FramesInFlight synchronizes N frames in flight.
    pub frames_in_flight: FramesInFlight,

    /// The application's swapchain.
    pub swapchain: Swapchain,

    /// The demo's cli args
    pub args: A,

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
        window: &mut glfw::Window,
        gfx: &mut Graphics<Self::Args>,
    ) -> Result<Self>
    where
        Self: Sized;

    /// Returns the dynamic rendering features required by this demo.
    fn physical_device_dynamic_rendering_features(
    ) -> vk::PhysicalDeviceDynamicRenderingFeatures<'static> {
        vk::PhysicalDeviceDynamicRenderingFeatures::default()
    }

    /// Returns the physical device features required by this demo.
    fn physical_device_features() -> vk::PhysicalDeviceFeatures {
        vk::PhysicalDeviceFeatures::default()
    }

    /// Returns the Vulkan 1.2 physical device features required by this demo.
    fn physical_device_vulkan12_features(
    ) -> vk::PhysicalDeviceVulkan12Features<'static> {
        vk::PhysicalDeviceVulkan12Features::default()
    }

    /// Returns the buffer device address features required by this demo.
    fn physical_device_buffer_device_address_features(
    ) -> vk::PhysicalDeviceBufferDeviceAddressFeatures<'static> {
        vk::PhysicalDeviceBufferDeviceAddressFeatures::default()
    }

    /// Handles a single GLFW event.
    ///
    /// This function is called in a loop to consume any pending events before
    /// every call to update().
    fn handle_event(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
        #[allow(unused_variables)] event: glfw::WindowEvent,
    ) -> Result<()> {
        if let glfw::WindowEvent::Key(
            glfw::Key::Escape,
            _,
            glfw::Action::Repeat,
            _,
        ) = event
        {
            window.set_should_close(true);
        }
        Ok(())
    }

    /// Called in a loop after all pending events have been processed.
    ///
    /// This is a good place for rendering logic. This method blocks event
    /// processing, so it should be kept as responsive as possible.
    fn update(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
    ) -> Result<()> {
        Ok(())
    }

    /// Build the command buffer for the next frame.
    ///
    /// Called after update() once the Frame is started.
    fn draw(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
        #[allow(unused_variables)] frame: &Frame,
    ) -> Result<()> {
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

        Ok(())
    }

    /// Rebuild any of the demo's swapchain-dependent resources.
    ///
    /// All pending frames in flight are guaranteed to have completed by the
    /// time this function is called. E.G. any previously-recorded Frame command
    /// buffers are guaranteed to be finished.
    fn rebuild_swapchain_resources(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
    ) -> Result<()> {
        Ok(())
    }

    /// Called when the application is paused.
    ///
    /// The application is paused automatically any time the framebuffer has
    /// size of 0. This occurs when the application is minimized, etc..
    fn paused(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
    ) -> Result<()> {
        Ok(())
    }

    /// Called when the application is unpaused.
    fn unpaused(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
    ) -> Result<()> {
        Ok(())
    }
}

struct DemoApp<D: Demo> {
    graphics: Graphics<D::Args>,
    demo: D,
}

impl<D: Demo> DemoApp<D> {
    /// Called any time the framebuffer size may have changed.
    /// Returns the current paused status for convenience.
    fn framebuffer_size_changed(
        &mut self,
        window: &mut glfw::Window,
    ) -> Result<bool> {
        let (w, h) = window.get_framebuffer_size();
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

    fn new(window: &mut glfw::Window, args: Self::Args) -> Result<Self>
    where
        Self: Sized,
    {
        window.set_size(D::INITIAL_WINDOW_SIZE.0, D::INITIAL_WINDOW_SIZE.1);
        window.set_title(std::any::type_name::<D>());

        let vulkan = VulkanContext::new(
            window,
            RequiredDeviceFeatures {
                physical_device_features: D::physical_device_features(),
                physical_device_vulkan12_features:
                    D::physical_device_vulkan12_features(),
                physical_device_dynamic_rendering_features:
                    D::physical_device_dynamic_rendering_features(),
                physical_device_buffer_device_address_features:
                    D::physical_device_buffer_device_address_features(),
            },
        )
        .with_context(trace!("Unable to create the Vulkan Context!"))?;

        let (w, h) = window.get_framebuffer_size();
        let swapchain =
            Swapchain::new(vulkan.clone(), (w as u32, h as u32), None)
                .with_context(trace!("Unable to create the swapchain!"))?;

        let frames_in_flight = FramesInFlight::new(
            vulkan.clone(),
            swapchain.images().len(),
            D::FRAMES_IN_FLIGHT_COUNT,
        )
        .with_context(trace!("Unable to create frames in flight!"))?;

        let fps_limiter = spin_sleep_util::interval(Duration::from_secs_f64(
            1.0 / D::FRAMES_PER_SECOND as f64,
        ));

        let mut graphics = Graphics {
            fps_limiter,
            vulkan,
            swapchain,
            frames_in_flight,
            args,
            metrics: FrameMetrics::new(D::FRAMES_PER_SECOND as usize),
            swapchain_needs_rebuild: false,
            paused: false,
        };
        let demo = D::new(window, &mut graphics)
            .with_context(trace!("Error initializing demo!"))?;

        Ok(Self { graphics, demo })
    }

    fn handle_event(
        &mut self,
        window: &mut glfw::Window,
        event: glfw::WindowEvent,
    ) -> Result<()> {
        self.demo.handle_event(window, &mut self.graphics, event)
    }

    fn update(&mut self, window: &mut glfw::Window) -> Result<()> {
        if self.graphics.paused && self.framebuffer_size_changed(window)? {
            std::hint::spin_loop();
            return Ok(());
        }

        if self.graphics.swapchain_needs_rebuild {
            self.graphics
                .frames_in_flight
                .wait_for_all_frames_to_complete()
                .with_context(trace!(
                    "Error waiting for frames before swapchain rebuild!"
                ))?;
            if self.framebuffer_size_changed(window)? {
                return Ok(());
            }
            let (w, h) = window.get_framebuffer_size();
            self.graphics.swapchain = Swapchain::new(
                self.graphics.vulkan.clone(),
                (w as u32, h as u32),
                Some(self.graphics.swapchain.raw()),
            )
            .with_context(trace!("Error while rebuilding swapchain!"))?;

            self.demo
                .rebuild_swapchain_resources(window, &mut self.graphics)
                .with_context(trace!(
                    "Error while rebuilding demo swapchain resources!"
                ))?;

            self.graphics.swapchain_needs_rebuild = false;
        }

        self.graphics.metrics.frame_tick();

        // Update application logic
        // ------------------------

        let before_update = Instant::now();
        self.demo
            .update(window, &mut self.graphics)
            .with_context(trace!("Unhandled error in Demo::update()!"))?;
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
                return Ok(());
            }
        };

        self.demo
            .draw(window, &mut self.graphics, &frame)
            .with_context(trace!("Unhandled error in Demo::draw()!"))?;

        let result = self
            .graphics
            .frames_in_flight
            .present_frame(&self.graphics.swapchain, frame)?;
        if result == PresentImageStatus::SwapchainNeedsRebuild {
            self.graphics.swapchain_needs_rebuild = true;
        }
        self.graphics.metrics.draw_tick(before_draw);

        Ok(())
    }
}

/// The main entrypoint for a demo.
pub fn demo_main<D: Demo + 'static>() -> Result<()> {
    app_main::<DemoApp<D>>()
}
