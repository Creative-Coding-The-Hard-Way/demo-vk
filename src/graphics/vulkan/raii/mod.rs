//! RAII wrappers for Vulkan objects.
//!
//! Wrappers do not track dependencies. The application is responsible for
//! dropping Vulkan objects in the correct order and synchronizing to prevent
//! GPU inconsistencies.

mod device;
mod device_extensions;
mod device_resources;
mod instance;
mod instance_extensions;

pub use self::{
    device::Device,
    device_extensions::Swapchain,
    device_resources::{CommandPool, ImageView, Semaphore},
    instance::Instance,
    instance_extensions::{DebugUtils, Surface},
};
