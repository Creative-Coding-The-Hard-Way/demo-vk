use {
    crate::graphics::vulkan::{raii, Instance, RequiredDeviceFeatures},
    anyhow::{Context, Result},
    ash::vk::{self, QueueFlags},
    std::sync::Arc,
};

/// Create the Vulkan device with all required features and queues for this
/// application.
pub fn create_logical_device(
    instance: &Instance,
    surface_khr: &raii::Surface,
    physical_device: vk::PhysicalDevice,
    required_device_features: RequiredDeviceFeatures,
) -> Result<(Arc<raii::Device>, u32)> {
    let queue_family_properties = unsafe {
        instance.get_physical_device_queue_family_properties(physical_device)
    };

    let (graphics_queue_index, _) = queue_family_properties
        .iter()
        .enumerate()
        .find(|(index, properties)| {
            let supports_present = unsafe {
                surface_khr
                    .ext
                    .get_physical_device_surface_support(
                        physical_device,
                        *index as u32,
                        surface_khr.raw,
                    )
                    .unwrap_or(false)
            };
            supports_present
                && properties.queue_flags.contains(QueueFlags::GRAPHICS)
        })
        .context("Unable to find a GRAPHICS device queue.")?;

    let queue_priorities = [1.0f32];
    let queue_create_infos = [vk::DeviceQueueCreateInfo {
        queue_family_index: graphics_queue_index as u32,
        queue_count: 1,
        p_queue_priorities: queue_priorities.as_ptr(),
        ..Default::default()
    }];
    let extensions = [ash::khr::swapchain::NAME.as_ptr()];

    let logical_device = {
        let mut physical_device_dynamic_rendering_features =
            vk::PhysicalDeviceDynamicRenderingFeatures {
                ..required_device_features
                    .physical_device_dynamic_rendering_features
            };
        let mut physical_device_vulkan12_features =
            vk::PhysicalDeviceVulkan12Features {
                ..required_device_features.physical_device_vulkan12_features
            };

        // pack the desired features
        let mut features = vk::PhysicalDeviceFeatures2 {
            features: vk::PhysicalDeviceFeatures {
                ..required_device_features.physical_device_features
            },
            ..Default::default()
        }
        .push_next(&mut physical_device_vulkan12_features)
        .push_next(&mut physical_device_dynamic_rendering_features);

        // create the device using the requested features
        let create_info = vk::DeviceCreateInfo {
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            p_enabled_features: std::ptr::null(), /* use physical device
                                                   * features2 */
            ..Default::default()
        }
        .push_next(&mut features);
        raii::Device::new(instance.ash.clone(), physical_device, &create_info)?
    };

    Ok((logical_device, graphics_queue_index as u32))
}
