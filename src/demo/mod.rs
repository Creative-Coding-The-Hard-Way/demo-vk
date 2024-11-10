use {
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
    std::sync::Arc,
};

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

    swapchain_needs_rebuild: bool,
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
}

struct DemoApp<D: Demo> {
    graphics: Graphics<D::Args>,
    demo: D,
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

        let mut graphics = Graphics {
            vulkan,
            swapchain,
            frames_in_flight,
            args,
            swapchain_needs_rebuild: false,
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
        if self.graphics.swapchain_needs_rebuild {
            self.graphics.swapchain_needs_rebuild = false;
            self.graphics
                .frames_in_flight
                .wait_for_all_frames_to_complete()
                .with_context(trace!(
                    "Error waiting for frames before swapchain rebuild!"
                ))?;
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
        }

        self.demo
            .update(window, &mut self.graphics)
            .with_context(trace!("Unhandled error in Demo::update()!"))?;

        // Start the next frame
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

        Ok(())
    }
}

/// The main entrypoint for a demo.
pub fn demo_main<D: Demo + 'static>() {
    app_main::<DemoApp<D>>();
}
