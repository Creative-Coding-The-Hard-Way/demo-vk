use {
    anyhow::Result,
    ash::vk,
    clap::Parser,
    demo_vk::{
        app::{app_main, App},
        graphics::vulkan::{
            FrameStatus, FramesInFlight, PresentImageStatus, Swapchain,
            VulkanContext,
        },
    },
    std::sync::Arc,
};

/// All apps accept arguments from the CLI via a type that implements the clap
/// Parser.
#[derive(Debug, Parser)]
struct Args {}

/// An app is just a struct. It can contain state and clean itself up in drop if
/// needed.
#[derive(Debug)]
struct VulkanApp {
    frames_in_flight: FramesInFlight,

    swapchain: Swapchain,
    swapchain_needs_rebuild: bool,

    ctx: Arc<VulkanContext>,
}

impl App for VulkanApp {
    type Args = Args;

    /// Create a new instance of the app.
    /// The glfw window is mutable and can be modified to suit the application's
    /// needs.
    fn new(window: &mut glfw::Window, _args: Self::Args) -> Result<Self> {
        window.set_all_polling(true);
        window.set_title(std::any::type_name::<Self>());

        let ctx = VulkanContext::new(
            window,
            vk::PhysicalDeviceFeatures::default(),
            vk::PhysicalDeviceVulkan12Features::default(),
            vk::PhysicalDeviceDynamicRenderingFeatures::default(),
        )?;

        let (w, h) = window.get_framebuffer_size();
        let swapchain =
            Swapchain::new(ctx.clone(), (w as u32, h as u32), None)?;

        let frames_in_flight =
            FramesInFlight::new(ctx.clone(), swapchain.images().len(), 5)?;

        log::info!(
            "Setup complete!\n{:#?}\n{:#?}\n{:#?}",
            ctx,
            swapchain,
            frames_in_flight
        );

        Ok(Self {
            ctx,
            swapchain,
            frames_in_flight,
            swapchain_needs_rebuild: false,
        })
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
    fn update(&mut self, window: &mut glfw::Window) -> Result<()> {
        // Rebuild the swapchain if needed
        if self.swapchain_needs_rebuild {
            log::info!("Rebuilding Swapchain");
            self.swapchain_needs_rebuild = false;
            self.frames_in_flight.wait_for_all_frames_to_complete()?;
            let (w, h) = window.get_framebuffer_size();
            self.swapchain = Swapchain::new(
                self.ctx.clone(),
                (w as u32, h as u32),
                Some(self.swapchain.raw()),
            )?;
        }

        // Start the next frame
        let frame = match self.frames_in_flight.start_frame(&self.swapchain)? {
            FrameStatus::FrameStarted(frame) => frame,
            FrameStatus::SwapchainNeedsRebuild => {
                log::info!("Swapchain Needs Rebuild");
                self.swapchain_needs_rebuild = true;
                return Ok(());
            }
        };

        // The swapchain image needs to transition to be presentable. This can
        // be done with a renderpass or, in this case, with an image memory
        // barrier.

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
            self.ctx.cmd_pipeline_barrier(
                frame.command_buffer(),
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        // finish the frame
        let status = self
            .frames_in_flight
            .present_frame(&self.swapchain, frame)?;

        if status == PresentImageStatus::SwapchainNeedsRebuild {
            log::info!("Swapchain Needs Rebuild");
            self.swapchain_needs_rebuild = true;
        }

        Ok(())
    }
}

pub fn main() {
    // app_main creates an instance of the app and starts the GLFW loop to
    // process events, etc...
    let _ = app_main::<VulkanApp>();
}
