//! This module defines traits, structs, and functions to interact with a Vulkan
//! device.
//!
//! The primary entrypoint is the [VulkanContext] which can be initialized with
//! a [glfw::Window].
//!
//! Most applications use the [FramesInFlight] to manage double/triple buffered
//! rendering as it handles the synchronization with the swapchain and provides
//! the useful [Frame] abstraction for submitting a freshly recorded
//! CommandBuffer on each frame.

mod allocator;
mod buffers;
mod context;
mod frames_in_flight;
pub mod raii;
mod spirv;
mod swapchain;
mod sync_commands;

pub use self::{
    allocator::{block::Block, owned_block::OwnedBlock, Allocator},
    buffers::{CPUBuffer, UniformBuffer},
    context::{Instance, RequiredDeviceFeatures, VulkanContext},
    frames_in_flight::{Frame, FrameStatus, FramesInFlight},
    spirv::{spirv_module, spirv_words},
    swapchain::{AcquireImageStatus, PresentImageStatus, Swapchain},
    sync_commands::SyncCommands,
};
