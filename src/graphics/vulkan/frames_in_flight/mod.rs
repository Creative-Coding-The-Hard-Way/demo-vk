use {
    crate::{
        graphics::vulkan::{
            raii, AcquireImageStatus, PresentImageStatus, Swapchain,
            VulkanContext,
        },
        unwrap_here,
    },
    anyhow::{Context, Result},
    ash::vk::{self, Handle},
    std::{ffi::CString, sync::Arc},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FrameStatus {
    /// Indicates that the frame is started.
    ///
    /// The command buffer is still owned by the FramesInFlight and does not
    /// need to be freed by the caller.
    FrameStarted(Frame),

    /// Indicates that the swapchain needs to be rebuilt.
    SwapchainNeedsRebuild,
}

/// A Frame is guaranteed to be synchronized such that no two frames with the
/// same frame_index can be in-flight on the GPU at the same time.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Frame {
    command_buffer: vk::CommandBuffer,
    swapchain_image_index: u32,
    frame_index: usize,
    swapchain_image: vk::Image,
    swapchain_image_view: vk::ImageView,
}

impl Frame {
    pub fn command_buffer(&self) -> vk::CommandBuffer {
        self.command_buffer
    }

    pub fn swapchain_image_index(&self) -> u32 {
        self.swapchain_image_index
    }

    pub fn frame_index(&self) -> usize {
        self.frame_index
    }

    pub fn swapchain_image(&self) -> vk::Image {
        self.swapchain_image
    }

    pub fn swapchain_image_view(&self) -> vk::ImageView {
        self.swapchain_image_view
    }
}

/// Used to track the status of each [FrameSync] instance.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum FrameSyncStatus {
    /// Indicates that the frame is currently being assembled. e.g. someone
    /// called start_frame, acquired this frame, and has not yet called
    /// present_frame.
    Assembling,

    /// Indicates that present_frame was called and the frame is in use by the
    /// GPU.
    Pending,
}

/// Per-frame synchronization primitives.
#[derive(Debug)]
struct FrameSync {
    status: FrameSyncStatus,
    swapchain_image_acquired: raii::Semaphore,
    graphics_commands_complete: raii::Fence,
    command_pool: raii::CommandPool,
    command_buffer: vk::CommandBuffer,
}

/// The fundamental synchronization mechanism for an application with N "frames
/// in flight" where frame K's graphics command buffer can be recorded while
/// frames K+1, K+2, ... K+N are all in-progress rendering on the GPU.
///
/// For example, when there are 3 frames in flight, the sequence can be
/// visualized like so:
#[doc = simple_mermaid::mermaid!("./frames_in_flight_diagram.mmd")]
///
/// The application takes ownership of the Frame object when calling
/// [Self::start_frame] and returns ownership when calling
/// [Self::present_frame]. As such, any time a system needs to access a
/// frame-specific resource, the best option is to accept an [Frame] borrow and
/// use [Frame::frame_index] to select frame-specific resources. This is certain
/// to be safe because the only time the application has a [Frame] instance is
/// when that frame's previous commands have finished executing.
#[derive(Debug)]
pub struct FramesInFlight {
    // Used to synchronize the calls to QueueSubmit and Present per swapchain
    // image.
    swapchain_image_present_semaphores: Vec<raii::Semaphore>,

    frames: Vec<FrameSync>,
    frame_index: usize,
    cxt: Arc<VulkanContext>,
}

impl FramesInFlight {
    /// Creates a new instance with `frame_count` frames.
    pub fn new(
        ctx: Arc<VulkanContext>,
        swapchain_image_count: usize,
        frame_count: usize,
    ) -> Result<Self> {
        // Create one semaphore per swapchain image
        let mut swapchain_image_present_semaphores =
            Vec::with_capacity(swapchain_image_count);
        for i in 0..swapchain_image_count {
            swapchain_image_present_semaphores.push(unwrap_here!(
                format!("Create present semaphore for swapchain image [{}]", i),
                raii::Semaphore::new(
                    format!("Swapchain Image Present [{}]", i),
                    ctx.device.clone(),
                    &vk::SemaphoreCreateInfo::default(),
                )
            ));
        }

        // create the per-frame synchronization primitives
        let mut frames = Vec::with_capacity(frame_count);
        for index in 0..frame_count {
            let command_pool = unwrap_here!(
                format!("Create command pool for frame {}", index),
                raii::CommandPool::new(
                    format!("frame [{}]", index),
                    ctx.device.clone(),
                    &vk::CommandPoolCreateInfo {
                        flags: vk::CommandPoolCreateFlags::TRANSIENT,
                        queue_family_index: ctx.graphics_queue_family_index,
                        ..Default::default()
                    },
                )
            );
            let command_buffer = unwrap_here!(
                format!("Create command buffer for frame {}", index),
                unsafe {
                    ctx.allocate_command_buffers(
                        &vk::CommandBufferAllocateInfo {
                            command_pool: command_pool.raw,
                            level: vk::CommandBufferLevel::PRIMARY,
                            command_buffer_count: 1,
                            ..Default::default()
                        },
                    )?
                    .first()
                    .copied()
                    .context("Expected exactly one command buffer")
                }
            );
            let buffer_name =
                CString::new(format!("Frame [{}] Commands", index)).unwrap();
            ctx.device
                .set_debug_name(&vk::DebugUtilsObjectNameInfoEXT {
                    object_type: vk::ObjectType::COMMAND_BUFFER,
                    object_handle: command_buffer.as_raw(),
                    p_object_name: buffer_name.as_ptr(),
                    ..Default::default()
                })?;
            frames.push(FrameSync {
                status: FrameSyncStatus::Pending,
                swapchain_image_acquired: unwrap_here!(
                    format!(
                        "Create image acquired semaphore for frame {index}"
                    ),
                    raii::Semaphore::new(
                        format!("image acquired - frame [{}]", index),
                        ctx.device.clone(),
                        &vk::SemaphoreCreateInfo::default(),
                    )
                ),
                graphics_commands_complete: unwrap_here!(
                    format!("Create graphics cmd fence for frame {index}"),
                    raii::Fence::new(
                        format!(
                            "graphics commands complete - frame [{}]",
                            index
                        ),
                        ctx.device.clone(),
                        &vk::FenceCreateInfo {
                            flags: vk::FenceCreateFlags::SIGNALED,
                            ..Default::default()
                        },
                    )
                ),
                command_pool,
                command_buffer,
            });
        }
        Ok(Self {
            swapchain_image_present_semaphores,
            frames,
            frame_index: 0,
            cxt: ctx,
        })
    }

    /// Rebuilds the swapchain semaphores.
    ///
    /// This should only be required after the swapchain is rebuilt.
    ///
    /// Rebuilding semaphores cannot be done until all images have finished,
    /// which means waiting for a full pipeline stall.
    pub fn rebuild_swapchain_semaphores(
        &mut self,
        ctx: &VulkanContext,
        swapchain_image_count: usize,
    ) -> Result<()> {
        self.wait_for_all_frames_to_complete()?;

        // Needed to ensure that all resources are finished being used before
        // continuing
        unsafe { ctx.device_wait_idle()? };

        self.swapchain_image_present_semaphores.clear();
        // Create one semaphore per swapchain image
        for i in 0..swapchain_image_count {
            self.swapchain_image_present_semaphores.push(unwrap_here!(
                format!("Create present semaphore for swapchain image [{}]", i),
                raii::Semaphore::new(
                    format!("Swapchain Image Present [{}]", i),
                    ctx.device.clone(),
                    &vk::SemaphoreCreateInfo::default(),
                )
            ));
        }

        for (i, frame_sync) in self.frames.iter_mut().enumerate() {
            frame_sync.swapchain_image_acquired = unwrap_here!(
                format!("Rebuild frame {i} swapchain image acquired semaphore"),
                raii::Semaphore::new(
                    format!("Swapchain Image Present [{}]", i),
                    ctx.device.clone(),
                    &vk::SemaphoreCreateInfo::default(),
                )
            );
        }

        Ok(())
    }

    /// Get the total number of configured frames in flight.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Blocks until all submitted commands for all frames have completed.
    ///
    /// NOTE: Only waits on pending frames. If there's a frame mid-assembly,
    ///       there's nothing to wait on. (and what's more, attempting to wait
    ///       would never succeed until the frame is submitted)
    pub fn wait_for_all_frames_to_complete(&self) -> Result<()> {
        let fences = self
            .frames
            .iter()
            .filter(|frame_sync| frame_sync.status == FrameSyncStatus::Pending)
            .map(|frame_sync| frame_sync.graphics_commands_complete.raw)
            .collect::<Vec<vk::Fence>>();
        unsafe {
            self.cxt
                .wait_for_fences(&fences, true, u64::MAX)
                .context("wait for all pending frames to complete")
        }
    }

    /// Starts the next frame in flight.
    ///
    /// This method *can* block if all frames are in flight. It will block until
    /// the next frame is available.
    ///
    /// # Returns
    ///
    /// A [FrameStatus] containing one of:
    /// * A [Frame] that must be returned to [Self::present_frame]
    /// * A flag indicating that the Swapchain needs to be rebuilt before the
    ///   next frame.
    pub fn start_frame(
        &mut self,
        swapchain: &Swapchain,
    ) -> Result<FrameStatus> {
        self.frame_index = (self.frame_index + 1) % self.frames.len();

        let frame_sync = &mut self.frames[self.frame_index];

        // Wait for the last frame's submission to complete, if its still
        // running.
        unwrap_here!("Wait for the previous submission to complete", unsafe {
            self.cxt.wait_for_fences(
                &[frame_sync.graphics_commands_complete.raw],
                true,
                u64::MAX,
            )
        });

        // Acquire the next Swapchain image
        let status = unwrap_here!(
            "Acquire the next swapchain image",
            swapchain.acquire_image(frame_sync.swapchain_image_acquired.raw)
        );
        let swapchain_image_index = match status {
            AcquireImageStatus::ImageAcquired(index) => index,
            _ => {
                return Ok(FrameStatus::SwapchainNeedsRebuild);
            }
        };

        // NOTE: only reset the frame's fence and command buffer _after_
        // acquiring a swapchain image. The order matters because the frame
        // will not be processed if the swapchain is out-of-date and needs
        // to be reconstructed.

        // Swapchain image is available, so reset the commands fence.
        unwrap_here!("Reset the frame's fence", unsafe {
            self.cxt
                .reset_fences(&[frame_sync.graphics_commands_complete.raw])
        });
        // mark the frame as pending so nobody gets stuck waiting for it
        frame_sync.status = FrameSyncStatus::Assembling;

        // Start the Frame's command buffer.
        unwrap_here!("Reset the frame's command pool", unsafe {
            self.cxt.reset_command_pool(
                frame_sync.command_pool.raw,
                vk::CommandPoolResetFlags::empty(),
            )
        });
        unwrap_here!("Begin the frame's command buffer", unsafe {
            self.cxt.begin_command_buffer(
                frame_sync.command_buffer,
                &vk::CommandBufferBeginInfo {
                    flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                    ..Default::default()
                },
            )
        });

        Ok(FrameStatus::FrameStarted(Frame {
            command_buffer: frame_sync.command_buffer,
            swapchain_image_index,
            frame_index: self.frame_index,
            swapchain_image: swapchain.images()[swapchain_image_index as usize],
            swapchain_image_view: swapchain.image_views()
                [swapchain_image_index as usize]
                .raw,
        }))
    }

    /// Queues the [Frame]'s command buffer and swapchain presentation.
    pub fn present_frame(
        &mut self,
        swapchain: &Swapchain,
        frame: Frame,
    ) -> Result<PresentImageStatus> {
        let frame_sync = &mut self.frames[frame.frame_index()];
        unwrap_here!("End the frame's command buffer", unsafe {
            self.cxt.end_command_buffer(frame_sync.command_buffer)
        });

        let wait_stage = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        unwrap_here!("Submit the frame's primary command buffer", unsafe {
            self.cxt.queue_submit(
                self.cxt.graphics_queue,
                &[vk::SubmitInfo {
                    wait_semaphore_count: 1,
                    p_wait_semaphores: &frame_sync.swapchain_image_acquired.raw,
                    p_wait_dst_stage_mask: &wait_stage,
                    command_buffer_count: 1,
                    p_command_buffers: &frame_sync.command_buffer,
                    signal_semaphore_count: 1,
                    p_signal_semaphores: &self
                        .swapchain_image_present_semaphores
                        [frame.swapchain_image_index as usize]
                        .raw,
                    ..Default::default()
                }],
                frame_sync.graphics_commands_complete.raw,
            )
        });
        frame_sync.status = FrameSyncStatus::Pending;

        swapchain.present_image(
            self.swapchain_image_present_semaphores
                [frame.swapchain_image_index as usize]
                .raw,
            frame.swapchain_image_index(),
        )
    }
}

impl Drop for FramesInFlight {
    fn drop(&mut self) {
        self.wait_for_all_frames_to_complete().unwrap();
        unsafe {
            self.cxt.device_wait_idle().unwrap();
        }
    }
}
