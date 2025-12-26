use {
    crate::{
        graphics::vulkan::{raii, Instance, RequiredDeviceFeatures},
        unwrap_here,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// Select a physical device based on the application's requried features and
/// properties.
pub fn pick_suitable_device(
    instance: &Instance,
    surface_khr: &raii::Surface,
    required_device_features: &RequiredDeviceFeatures,
) -> Result<vk::PhysicalDevice> {
    let physical_devices = unwrap_here!("Enumerate physical devices", unsafe {
        instance.enumerate_physical_devices()
    });

    log::trace!("Searching for suitable physical device...");

    let mut preferred_device = None;

    for physical_device in physical_devices {
        let properties = {
            let mut physical_device_properties =
                vk::PhysicalDeviceProperties2::default();
            unsafe {
                instance.get_physical_device_properties2(
                    physical_device,
                    &mut physical_device_properties,
                );
            }
            physical_device_properties.properties
        };
        let name = properties.device_name_as_c_str().unwrap_or_default();

        log::trace!("Check device {:?}", name);
        log::trace!("Device properties: {:#?}", properties);

        let has_features = has_required_features(
            instance,
            physical_device,
            required_device_features,
        );
        let has_queues = has_required_queues(instance, physical_device);
        let has_extensions = has_required_extensions(instance, physical_device);
        let has_surface_formats = unwrap_here!(
            "Check physical device surface formats",
            has_required_surface_formats(surface_khr, physical_device)
        );

        log::trace!(
            indoc::indoc! {"
                Device: {:?}
                 - has_required_features: {}
                 - has_required_queues: {}
                 - has_required_extensions: {}
                 - has_required_surface_formats: {}
            "},
            name,
            has_features,
            has_queues,
            has_extensions,
            has_surface_formats,
        );

        if has_features
            && has_queues
            && has_extensions
            && has_surface_formats
            && (preferred_device.is_none()
                || properties.device_type
                    == vk::PhysicalDeviceType::DISCRETE_GPU)
        {
            preferred_device = Some(physical_device)
        }
    }

    preferred_device.context("No suitable physical device could be found!")
}

fn has_required_queues(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let queue_propertes: Vec<vk::QueueFamilyProperties> = {
        let len = unsafe {
            instance.get_physical_device_queue_family_properties2_len(
                physical_device,
            )
        };
        let mut physical_device_queue_properties =
            vec![vk::QueueFamilyProperties2::default(); len];
        unsafe {
            instance.get_physical_device_queue_family_properties2(
                physical_device,
                &mut physical_device_queue_properties,
            );
        }
        physical_device_queue_properties
            .iter()
            .map(|properties| properties.queue_family_properties)
            .collect()
    };
    log::trace!("{:#?}", queue_propertes);

    queue_propertes.iter().any(|properties| {
        properties.queue_flags.contains(vk::QueueFlags::GRAPHICS)
    })
}

fn has_required_extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let extension_properties = unsafe {
        instance
            .enumerate_device_extension_properties(physical_device)
            .unwrap_or_default()
    };
    log::trace!("{:#?}", extension_properties);

    extension_properties.iter().any(|props| {
        props.extension_name_as_c_str().unwrap_or_default()
            == ash::khr::swapchain::NAME
    })
}

fn has_required_surface_formats(
    surface_khr: &raii::Surface,
    physical_device: vk::PhysicalDevice,
) -> Result<bool> {
    let formats = unsafe {
        surface_khr.ext.get_physical_device_surface_formats(
            physical_device,
            surface_khr.raw,
        )?
    };
    log::trace!("{:#?}", formats);

    let present_modes = unsafe {
        surface_khr.ext.get_physical_device_surface_present_modes(
            physical_device,
            surface_khr.raw,
        )?
    };
    log::trace!("{:#?}", present_modes);

    Ok(!formats.is_empty() && !present_modes.is_empty())
}

/// Returns true when the listed physical device has the features required by
/// the application.
fn has_required_features(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    required_device_features: &RequiredDeviceFeatures,
) -> bool {
    // load supported fetaures from the device
    let mut actual_dynamic_rendering_features =
        vk::PhysicalDeviceDynamicRenderingFeatures::default();
    let mut actual_vulkan12_features =
        vk::PhysicalDeviceVulkan12Features::default();
    let actual_features = unsafe {
        let mut features = vk::PhysicalDeviceFeatures2::default()
            .push_next(&mut actual_vulkan12_features)
            .push_next(&mut actual_dynamic_rendering_features);
        instance.get_physical_device_features2(physical_device, &mut features);
        features.features
    };

    macro_rules! check {
        ($desired:expr, $actual:ident, $name:ident) => {
            if $desired.$name == vk::TRUE && $actual.$name != vk::TRUE {
                log::warn!(
                    "{} not supported! Wanted {} but was {}",
                    stringify!($name),
                    $desired.$name,
                    $actual.$name
                );
                return false;
            }
        };
    }

    // check for dynamic rendering support
    check!(
        required_device_features.physical_device_dynamic_rendering_features,
        actual_dynamic_rendering_features,
        dynamic_rendering
    );

    macro_rules! check_feature {
        ($name:ident) => {
            check!(
                required_device_features.physical_device_features,
                actual_features,
                $name
            )
        };
    }
    check_feature!(robust_buffer_access);
    check_feature!(full_draw_index_uint32);
    check_feature!(image_cube_array);
    check_feature!(independent_blend);
    check_feature!(geometry_shader);
    check_feature!(tessellation_shader);
    check_feature!(sample_rate_shading);
    check_feature!(dual_src_blend);
    check_feature!(logic_op);
    check_feature!(multi_draw_indirect);
    check_feature!(draw_indirect_first_instance);
    check_feature!(depth_clamp);
    check_feature!(depth_bias_clamp);
    check_feature!(fill_mode_non_solid);
    check_feature!(depth_bounds);
    check_feature!(wide_lines);
    check_feature!(large_points);
    check_feature!(alpha_to_one);
    check_feature!(multi_viewport);
    check_feature!(sampler_anisotropy);
    check_feature!(texture_compression_etc2);
    check_feature!(texture_compression_astc_ldr);
    check_feature!(texture_compression_bc);
    check_feature!(occlusion_query_precise);
    check_feature!(pipeline_statistics_query);
    check_feature!(vertex_pipeline_stores_and_atomics);
    check_feature!(fragment_stores_and_atomics);
    check_feature!(shader_tessellation_and_geometry_point_size);
    check_feature!(shader_image_gather_extended);
    check_feature!(shader_storage_image_extended_formats);
    check_feature!(shader_storage_image_multisample);
    check_feature!(shader_storage_image_read_without_format);
    check_feature!(shader_storage_image_write_without_format);
    check_feature!(shader_uniform_buffer_array_dynamic_indexing);
    check_feature!(shader_sampled_image_array_dynamic_indexing);
    check_feature!(shader_storage_buffer_array_dynamic_indexing);
    check_feature!(shader_storage_image_array_dynamic_indexing);
    check_feature!(shader_clip_distance);
    check_feature!(shader_cull_distance);
    check_feature!(shader_float64);
    check_feature!(shader_int64);
    check_feature!(shader_int16);
    check_feature!(shader_resource_residency);
    check_feature!(shader_resource_min_lod);
    check_feature!(sparse_binding);
    check_feature!(sparse_residency_buffer);
    check_feature!(sparse_residency_image2_d);
    check_feature!(sparse_residency_image3_d);
    check_feature!(sparse_residency2_samples);
    check_feature!(sparse_residency4_samples);
    check_feature!(sparse_residency8_samples);
    check_feature!(sparse_residency16_samples);
    check_feature!(sparse_residency_aliased);
    check_feature!(variable_multisample_rate);
    check_feature!(inherited_queries);

    macro_rules! check_feature12 {
        ($name:ident) => {
            check!(
                required_device_features.physical_device_vulkan12_features,
                actual_vulkan12_features,
                $name
            )
        };
    }
    check_feature12!(sampler_mirror_clamp_to_edge);
    check_feature12!(draw_indirect_count);
    check_feature12!(storage_buffer8_bit_access);
    check_feature12!(uniform_and_storage_buffer8_bit_access);
    check_feature12!(storage_push_constant8);
    check_feature12!(shader_buffer_int64_atomics);
    check_feature12!(shader_shared_int64_atomics);
    check_feature12!(shader_float16);
    check_feature12!(shader_int8);
    check_feature12!(descriptor_indexing);
    check_feature12!(shader_input_attachment_array_dynamic_indexing);
    check_feature12!(shader_uniform_texel_buffer_array_dynamic_indexing);
    check_feature12!(shader_storage_texel_buffer_array_dynamic_indexing);
    check_feature12!(shader_uniform_buffer_array_non_uniform_indexing);
    check_feature12!(shader_sampled_image_array_non_uniform_indexing);
    check_feature12!(shader_storage_buffer_array_non_uniform_indexing);
    check_feature12!(shader_storage_image_array_non_uniform_indexing);
    check_feature12!(shader_input_attachment_array_non_uniform_indexing);
    check_feature12!(shader_uniform_texel_buffer_array_non_uniform_indexing);
    check_feature12!(shader_storage_texel_buffer_array_non_uniform_indexing);
    check_feature12!(descriptor_binding_uniform_buffer_update_after_bind);
    check_feature12!(descriptor_binding_sampled_image_update_after_bind);
    check_feature12!(descriptor_binding_storage_image_update_after_bind);
    check_feature12!(descriptor_binding_storage_buffer_update_after_bind);
    check_feature12!(descriptor_binding_uniform_texel_buffer_update_after_bind);
    check_feature12!(descriptor_binding_storage_texel_buffer_update_after_bind);
    check_feature12!(descriptor_binding_update_unused_while_pending);
    check_feature12!(descriptor_binding_partially_bound);
    check_feature12!(descriptor_binding_variable_descriptor_count);
    check_feature12!(runtime_descriptor_array);
    check_feature12!(sampler_filter_minmax);
    check_feature12!(scalar_block_layout);
    check_feature12!(imageless_framebuffer);
    check_feature12!(uniform_buffer_standard_layout);
    check_feature12!(shader_subgroup_extended_types);
    check_feature12!(separate_depth_stencil_layouts);
    check_feature12!(host_query_reset);
    check_feature12!(timeline_semaphore);
    check_feature12!(buffer_device_address);
    check_feature12!(buffer_device_address_capture_replay);
    check_feature12!(buffer_device_address_multi_device);
    check_feature12!(vulkan_memory_model);
    check_feature12!(vulkan_memory_model_device_scope);
    check_feature12!(vulkan_memory_model_availability_visibility_chains);
    check_feature12!(shader_output_viewport_index);
    check_feature12!(shader_output_layer);
    check_feature12!(subgroup_broadcast_dynamic_id);

    true
}
