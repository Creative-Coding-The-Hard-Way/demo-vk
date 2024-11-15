//! This module defines traits, structs, and functions to interact with a Vulkan
//! device.
//!
//! The primary entrypoint is the [VulkanContext] which can be initialized with
//! a [glfw::Window].

mod allocator;
mod buffers;
mod context;
mod descriptors;
mod frames_in_flight;
pub mod raii;
mod shader_compiler;
mod swapchain;
mod sync_commands;

pub use self::{
    allocator::{block::Block, owned_block::OwnedBlock, Allocator},
    buffers::{CPUBuffer, UniformBuffer},
    context::{Instance, VulkanContext},
    descriptors::{DescriptorBumpAllocator, PoolRatio},
    frames_in_flight::{Frame, FrameStatus, FramesInFlight},
    shader_compiler::{compile_slang, spirv_module, spirv_words},
    swapchain::{AcquireImageStatus, PresentImageStatus, Swapchain},
    sync_commands::SyncCommands,
};
