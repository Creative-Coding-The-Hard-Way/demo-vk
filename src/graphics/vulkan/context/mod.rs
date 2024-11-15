mod instance;
mod logical_device;
mod physical_device;

use {
    crate::{
        graphics::vulkan::{raii, Allocator},
        trace,
    },
    anyhow::{Context, Result},
    ash::vk::{self},
    std::sync::Arc,
};

pub use self::instance::Instance;

/// The Vulkan context is the logical handle for all Vulkan operations within
/// the app.
///
/// It supports finding and using a single logical device along with all
/// required queues and a device memory allocator.
pub struct VulkanContext {
    pub instance: Instance,
    pub surface_khr: Arc<raii::Surface>,
    pub physical_device: vk::PhysicalDevice,
    pub device: Arc<raii::Device>,

    /// The queue family index for the graphics + present queue.
    pub graphics_queue_family_index: u32,

    /// The graphics queue supports GRAPHICS and presentation operations.
    pub graphics_queue: vk::Queue,

    /// The device memory allocator.
    pub allocator: Arc<Allocator>,
}

impl VulkanContext {
    /// Creates a new Vulkan Context for the first suitable device that supports
    /// presenting to the GLFW window surface.
    pub fn new(window: &glfw::Window) -> Result<Arc<Self>> {
        let instance = Instance::for_window("Shader-Toy-Slang", window)
            .with_context(trace!("Unable to create vulkan instance!"))?;

        let surface_khr =
            raii::Surface::for_window(instance.ash.clone(), window)
                .with_context(trace!(
                    "Unable to create Vulkan surface from glfw window!"
                ))?;

        let physical_device =
            physical_device::pick_suitable_device(&instance, &surface_khr)
                .with_context(trace!(
                    "Error while picking a suitable physical device!"
                ))?;

        let (device, graphics_queue_family_index) =
            logical_device::create_logical_device(
                &instance,
                &surface_khr,
                physical_device,
            )
            .with_context(trace!("Error while creating the logical device!"))?;

        let graphics_queue =
            unsafe { device.get_device_queue(graphics_queue_family_index, 0) };

        let allocator = Allocator::new(device.clone(), physical_device)
            .with_context(trace!(
                "Error while creating device memory allocator!"
            ))?;

        Ok(Arc::new(Self {
            instance,
            surface_khr,
            physical_device,
            device,
            graphics_queue_family_index,
            graphics_queue,
            allocator: Arc::new(allocator),
        }))
    }
}

impl std::ops::Deref for VulkanContext {
    type Target = ash::Device;

    fn deref(&self) -> &Self::Target {
        &self.device.raw
    }
}

impl std::fmt::Debug for VulkanContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VulkanContext")
            .field("instance", &self.instance)
            .field("surface_khr", &self.surface_khr)
            .field("physical_device", &self.physical_device)
            .field("device", &self.device)
            .field(
                "graphics_queue_family_index",
                &self.graphics_queue_family_index,
            )
            .field("graphics_queue", &self.graphics_queue)
            .finish()
    }
}
