mod instance;
mod logical_device;
mod physical_device;

use {
    crate::{
        graphics::vulkan::{raii, Allocator},
        unwrap_here,
    },
    anyhow::Result,
    ash::vk::{self},
    std::sync::Arc,
    winit::window::Window,
};

pub use self::instance::Instance;

/// Holds all of the device features structs which can be used when creating a
/// VulkanContext.
///
/// Note: None of the pnext pointers should be specified in these structures.
///       The relevant pnext chain will be assembled on-demand when calling
///       into Vulkan and never before.
#[derive(Debug, Default)]
pub struct RequiredDeviceFeatures {
    pub physical_device_features: vk::PhysicalDeviceFeatures,
    pub physical_device_maintenance4_features:
        vk::PhysicalDeviceMaintenance4Features<'static>,
    pub physical_device_vulkan12_features:
        vk::PhysicalDeviceVulkan12Features<'static>,
    pub physical_device_dynamic_rendering_features:
        vk::PhysicalDeviceDynamicRenderingFeatures<'static>,
}

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
    pub fn new(
        window: &Window,
        required_device_features: RequiredDeviceFeatures,
    ) -> Result<Arc<Self>> {
        let instance = unwrap_here!(
            "Create Vulkan instance for the application window",
            Instance::for_window("demo-vk", window)
        );

        let surface_khr = unwrap_here!(
            "Create Vulkan surface for the application window",
            raii::Surface::for_window(instance.ash.clone(), window)
        );

        let physical_device = unwrap_here!(
            "Pick a suitable device for the application",
            physical_device::pick_suitable_device(
                &instance,
                &surface_khr,
                &required_device_features,
            )
        );

        let (device, graphics_queue_family_index) = unwrap_here!(
            "Create a logical device for the chosen physical device",
            logical_device::create_logical_device(
                &instance,
                &surface_khr,
                physical_device,
                required_device_features,
            )
        );

        let graphics_queue =
            unsafe { device.get_device_queue(graphics_queue_family_index, 0) };

        let allocator = unwrap_here!(
            "Create the Vulkan GPU memory allocator",
            Allocator::new(device.clone(), physical_device)
        );

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
