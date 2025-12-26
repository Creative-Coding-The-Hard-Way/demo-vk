use {
    anyhow::Result,
    ash::vk,
    clap::Parser,
    demo_vk::{
        app::{app_main, App, AppState},
        graphics::vulkan::{
            FrameStatus, FramesInFlight, PresentImageStatus,
            RequiredDeviceFeatures, Swapchain, VulkanContext,
        },
        unwrap_here,
    },
    std::sync::Arc,
    winit::{
        dpi::PhysicalSize,
        event::WindowEvent,
        keyboard::{KeyCode, PhysicalKey},
        window::Window,
    },
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
    fn new(window: &mut Window, _args: &Self::Args) -> Result<Self> {
        let ctx = unwrap_here!(
            "Create the Vulkan Context",
            VulkanContext::new(window, RequiredDeviceFeatures::default())
        );

        let PhysicalSize {
            width: w,
            height: h,
        } = window.inner_size();
        let swapchain = unwrap_here!(
            "Create the application swapchain",
            Swapchain::new(ctx.clone(), (w as u32, h as u32), None)
        );

        let frames_in_flight = unwrap_here!(
            "Create frames-in-flight synchronization",
            FramesInFlight::new(ctx.clone(), swapchain.images().len(), 2)
        );

        log::info!("Setup complete!");

        Ok(Self {
            ctx,
            swapchain,
            frames_in_flight,
            swapchain_needs_rebuild: false,
        })
    }

    /// Handle an event.
    fn handle_window_event(
        &mut self,
        _window: &mut Window,
        event: WindowEvent,
    ) -> Result<AppState> {
        // Pattern match the event and tell the window to close if any key is
        // pressed.
        if let WindowEvent::KeyboardInput {
            device_id: _,
            event,
            is_synthetic: _,
        } = &event
        {
            if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                println!("request close");
                return Ok(AppState::Exit);
            }
        }
        Ok(AppState::Continue)
    }

    /// Update the application.
    fn update(&mut self, window: &mut Window) -> Result<AppState> {
        // Rebuild the swapchain if needed
        if self.swapchain_needs_rebuild {
            log::info!("rebuilding swapchain");

            self.swapchain_needs_rebuild = false;
            unwrap_here!(
                "wait for all frames to finish before rebuilding the swapchain",
                self.frames_in_flight.wait_for_all_frames_to_complete()
            );
            let PhysicalSize {
                width: w,
                height: h,
            } = window.inner_size();
            self.swapchain = unwrap_here!(
                "rebuild the swapchain",
                Swapchain::new(self.ctx.clone(), (w as u32, h as u32), None,)
            );
        }

        // Start the next frame
        let frame = match self.frames_in_flight.start_frame(&self.swapchain)? {
            FrameStatus::FrameStarted(frame) => frame,
            FrameStatus::SwapchainNeedsRebuild => {
                log::info!("start frame - swapchain needs rebuild");
                self.swapchain_needs_rebuild = true;
                return Ok(AppState::Continue);
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
            log::info!("Present frame - swapchain needs rebuild");
            self.swapchain_needs_rebuild = true;
        }

        Ok(AppState::Continue)
    }
}

pub fn main() {
    // app_main creates an instance of the app and starts the GLFW loop to
    // process events, etc...
    let _ = app_main::<VulkanApp>();
}
