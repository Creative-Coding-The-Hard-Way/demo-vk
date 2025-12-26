use {
    crate::{
        graphics::vulkan::{raii, VulkanContext},
        unwrap_here,
    },
    anyhow::Result,
    ash::vk,
    std::sync::Arc,
};

/// A utility for synchronously submitting commands to the GPU.
#[derive(Debug)]
pub struct SyncCommands {
    command_pool: raii::CommandPool,
    command_buffer: vk::CommandBuffer,
    fence: raii::Fence,
    cxt: Arc<VulkanContext>,
}

impl SyncCommands {
    pub fn new(cxt: Arc<VulkanContext>) -> Result<Self> {
        let command_pool = unwrap_here!(
            "Create command pool",
            raii::CommandPool::new(
                "SyncCommands",
                cxt.device.clone(),
                &vk::CommandPoolCreateInfo {
                    flags: vk::CommandPoolCreateFlags::TRANSIENT,
                    queue_family_index: cxt.graphics_queue_family_index,
                    ..Default::default()
                },
            )
        );
        let command_buffer =
            unwrap_here!("Allocate primary command buffer", unsafe {
                cxt.allocate_command_buffers(&vk::CommandBufferAllocateInfo {
                    command_pool: command_pool.raw,
                    level: vk::CommandBufferLevel::PRIMARY,
                    command_buffer_count: 1,
                    ..Default::default()
                })?
                .first()
                .copied()
                .context("Expected exactly one command buffer to be returned!")
            });
        let fence = unwrap_here!(
            "Create command fence",
            raii::Fence::new(
                "SyncCommands",
                cxt.device.clone(),
                &vk::FenceCreateInfo::default(),
            )
        );
        Ok(Self {
            command_pool,
            command_buffer,
            fence,
            cxt,
        })
    }

    pub fn submit_and_wait(
        &self,
        build_commands: impl FnOnce(vk::CommandBuffer) -> Result<()>,
    ) -> Result<()> {
        unwrap_here!("Reset command pool", unsafe {
            self.cxt.reset_command_pool(
                self.command_pool.raw,
                vk::CommandPoolResetFlags::empty(),
            )
        });

        unwrap_here!("Begin command buffer one time submit", unsafe {
            self.cxt.begin_command_buffer(
                self.command_buffer,
                &vk::CommandBufferBeginInfo {
                    flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                    ..Default::default()
                },
            )
        });

        unwrap_here!(
            "Add commands to the buffer",
            build_commands(self.command_buffer)
        );

        unwrap_here!("End command buffer", unsafe {
            self.cxt.end_command_buffer(self.command_buffer)
        });

        unwrap_here!("Submit commands and signal fence", unsafe {
            self.cxt.queue_submit(
                self.cxt.graphics_queue,
                &[vk::SubmitInfo {
                    wait_semaphore_count: 0,
                    p_wait_semaphores: std::ptr::null(),
                    p_wait_dst_stage_mask: std::ptr::null(),
                    command_buffer_count: 1,
                    p_command_buffers: &self.command_buffer,
                    signal_semaphore_count: 0,
                    p_signal_semaphores: std::ptr::null(),
                    ..Default::default()
                }],
                self.fence.raw,
            )
        });

        unwrap_here!("Wait for submission fence", unsafe {
            self.cxt.wait_for_fences(&[self.fence.raw], true, u64::MAX)
        });

        unwrap_here!("Reset fence after commands complete", unsafe {
            self.cxt.reset_fences(&[self.fence.raw])
        });

        Ok(())
    }
}
