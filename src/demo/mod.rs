mod rolling_average;

use {
    self::rolling_average::RollingAverage,
    crate::{
        app::{app_main, App},
        graphics::vulkan::{
            Frame, FrameStatus, FramesInFlight, PresentImageStatus, Swapchain,
            VulkanContext,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
    clap::Parser,
    spin_sleep_util::Interval,
    std::{
        collections::BTreeMap,
        sync::Arc,
        time::{Duration, Instant},
    },
};

#[derive(Debug)]
pub struct FrameMetrics {
    target_fps: usize,
    last_frame: Instant,
    ms_per_frame: RollingAverage,
    ms_per_update: RollingAverage,
    ms_per_draw: RollingAverage,
    metrics: BTreeMap<String, RollingAverage>,
}

impl FrameMetrics {
    /// Creates a new FrameMetrics instance.
    fn new(target_fps: usize) -> Self {
        Self {
            target_fps,
            last_frame: Instant::now(),
            ms_per_frame: RollingAverage::new(
                target_fps,
                1.0 / target_fps as f32,
            ),
            ms_per_update: RollingAverage::new(
                target_fps,
                1.0 / target_fps as f32,
            ),
            ms_per_draw: RollingAverage::new(
                target_fps,
                1.0 / target_fps as f32,
            ),
            metrics: BTreeMap::new(),
        }
    }

    /// Called when the application is unpaused.
    ///
    /// Resets the last frame time so there isn't a single massively slow frame.
    fn unpause(&mut self) {
        self.last_frame = Instant::now();
    }

    /// Calculates the time since the last frame and update internal metrics.
    ///
    /// Returns the time since the last frame in seconds.
    fn frame_tick(&mut self) -> f32 {
        let now = Instant::now();
        let frame_seconds = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        self.ms_per_frame.push(frame_seconds * 1000.0);

        frame_seconds
    }

    fn update_tick(&mut self, before_update: Instant) {
        let ms =
            Instant::now().duration_since(before_update).as_secs_f32() * 1000.0;
        self.ms_per_update.push(ms);
    }

    fn draw_tick(&mut self, before_draw: Instant) {
        let ms =
            Instant::now().duration_since(before_draw).as_secs_f32() * 1000.0;
        self.ms_per_draw.push(ms);
    }

    /// Records the milliseconds from start_time until now() at the time this
    /// function is called.
    ///
    /// Returns the milliseconds since the start time.
    pub fn ms_since(
        &mut self,
        name: impl Into<String>,
        start_time: Instant,
    ) -> f32 {
        let ms =
            Instant::now().duration_since(start_time).as_secs_f32() * 1000.0;
        self.record_metric(name, ms);
        ms
    }

    /// Saves a duration to the frame metrics.
    pub fn record_metric(&mut self, name: impl Into<String>, value: f32) {
        let metric = self
            .metrics
            .entry(name.into())
            .or_insert_with(|| RollingAverage::new(self.target_fps, value));
        metric.push(value);
    }

    /// Resets all tracked metrics.
    pub fn reset_metrics(&mut self) {
        self.metrics.clear();
    }
}

impl std::fmt::Display for FrameMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        f.write_fmt(format_args!(
            indoc::indoc! {"
                Frame Metrics
                fps : {:.0}
                mspf: {:.*}
                mspu: {:.*}
                mspd: {:.*}
            "},
            1000.0 / self.ms_per_frame.average(),
            precision,
            self.ms_per_frame,
            precision,
            self.ms_per_update,
            precision,
            self.ms_per_draw,
        ))?;

        for (name, metric) in self.metrics.iter() {
            f.write_fmt(format_args!("{}: {:.*}\n", name, precision, metric))?;
        }

        Ok(())
    }
}

/// Standard graphics resources provided by the Demo.
pub struct Graphics<A> {
    /// The Vulkan entrypoint
    pub vulkan: Arc<VulkanContext>,

    /// The application's swapchain.
    pub swapchain: Swapchain,

    /// FramesInFlight synchronizes N frames in flight.
    pub frames_in_flight: FramesInFlight,

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

        let vulkan = VulkanContext::new(window)
            .with_context(trace!("Unable to create the Vulkan Context!"))?;

        let (w, h) = window.get_framebuffer_size();
        let swapchain =
            Swapchain::new(vulkan.clone(), (w as u32, h as u32), None)
                .with_context(trace!("Unable to create the swapchain!"))?;

        let frames_in_flight =
            FramesInFlight::new(vulkan.clone(), D::FRAMES_IN_FLIGHT_COUNT)
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
pub fn demo_main<D: Demo + 'static>() {
    app_main::<DemoApp<D>>();
}
