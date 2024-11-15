use {
    crate::{
        graphics::vulkan::{raii, Instance},
        trace,
    },
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
        .with_context(trace!(
            "Unable to find a queue that supports GRAPHICS."
        ))?;

    let queue_priorities = [1.0f32];
    let queue_create_infos = [vk::DeviceQueueCreateInfo {
        queue_family_index: graphics_queue_index as u32,
        queue_count: 1,
        p_queue_priorities: queue_priorities.as_ptr(),
        ..Default::default()
    }];
    let extensions = [ash::khr::swapchain::NAME.as_ptr()];

    let mut vulkan12_features = vk::PhysicalDeviceVulkan12Features {
        runtime_descriptor_array: vk::TRUE,
        shader_sampled_image_array_non_uniform_indexing: vk::TRUE,
        descriptor_binding_partially_bound: vk::TRUE,
        ..Default::default()
    };
    let mut features = vk::PhysicalDeviceFeatures2 {
        features: vk::PhysicalDeviceFeatures {
            sampler_anisotropy: vk::TRUE,
            ..Default::default()
        },
        ..Default::default()
    }
    .push_next(&mut vulkan12_features);
    let create_info = vk::DeviceCreateInfo {
        queue_create_info_count: queue_create_infos.len() as u32,
        p_queue_create_infos: queue_create_infos.as_ptr(),
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_extension_names: extensions.as_ptr(),
        p_enabled_features: std::ptr::null(), // use physical device features2
        ..Default::default()
    }
    .push_next(&mut features);
    let logical_device =
        raii::Device::new(instance.ash.clone(), physical_device, &create_info)?;

    Ok((logical_device, graphics_queue_index as u32))
}
