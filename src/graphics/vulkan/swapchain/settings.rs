use {
    crate::{
        graphics::vulkan::{raii, VulkanContext},
        unwrap_here,
    },
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

pub fn create_swapchain(
    cxt: &VulkanContext,
    framebuffer_size: (u32, u32),
    previous_swapchain: Option<vk::SwapchainKHR>,
) -> Result<(Arc<raii::Swapchain>, vk::Extent2D, vk::SurfaceFormatKHR)> {
    let capabilities =
        unwrap_here!("Get device surface capabilities", unsafe {
            cxt.surface_khr
                .ext
                .get_physical_device_surface_capabilities(
                    cxt.physical_device,
                    cxt.surface_khr.raw,
                )
        });
    log::trace!("Device capabilities:\n{:#?}", capabilities);

    let format =
        unwrap_here!("Select surface image format", select_image_format(cxt));
    let extent = select_image_extent(&capabilities, framebuffer_size);
    let queue_families = [cxt.graphics_queue_family_index];
    let create_info = vk::SwapchainCreateInfoKHR {
        surface: cxt.surface_khr.raw,
        min_image_count: select_image_count(&capabilities),
        image_format: format.format,
        image_color_space: format.color_space,
        image_extent: extent,
        image_array_layers: 1,
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT
            | vk::ImageUsageFlags::TRANSFER_DST,
        image_sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 1,
        p_queue_family_indices: queue_families.as_ptr(),
        pre_transform: capabilities.current_transform,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode: unwrap_here!(
            "Select surface present mode",
            select_present_mode(cxt)
        ),
        clipped: vk::TRUE,
        old_swapchain: previous_swapchain.unwrap_or(vk::SwapchainKHR::null()),
        ..Default::default()
    };
    let swapchain = unwrap_here!(
        "Create new swapchain",
        raii::Swapchain::new(cxt.device.clone(), &create_info)
    );
    log::trace!(
        indoc::indoc!(
            "
            Created swapchain:
              - swapchain: {:#?}
              - extent: {:?}
              - format: {:?}
            "
        ),
        swapchain,
        extent,
        format
    );
    Ok((swapchain, extent, format))
}

/// Pick the desired image format for the swapchain.
fn select_image_format(cxt: &VulkanContext) -> Result<vk::SurfaceFormatKHR> {
    let surface_formats =
        unwrap_here!("List avialable surface formats", unsafe {
            cxt.surface_khr.ext.get_physical_device_surface_formats(
                cxt.physical_device,
                cxt.surface_khr.raw,
            )
        });
    log::trace!("Formats supported by device\n{:#?}", surface_formats);

    let preferred = surface_formats.iter().find(|surface_format| {
        let has_color_space =
            surface_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR;
        let has_format = surface_format.format == vk::Format::B8G8R8A8_SRGB;
        has_color_space && has_format
    });

    let format = preferred.or(surface_formats.first()).context(
        "Unable to find a suitable surface format for the swapchain!",
    )?;

    Ok(*format)
}

fn select_image_count(capabilities: &vk::SurfaceCapabilitiesKHR) -> u32 {
    let count = capabilities.min_image_count + 2;
    if capabilities.max_image_count > 0 {
        count.clamp(capabilities.min_image_count, capabilities.max_image_count)
    } else {
        count
    }
}

fn select_image_extent(
    capabilities: &vk::SurfaceCapabilitiesKHR,
    framebuffer_size: (u32, u32),
) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        // If current_extent does not equal u32::MAX, then it contains the
        // extent of the targeted surface and can be used directly. See:
        //
        // https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html

        return capabilities.current_extent;
    }

    let (desired_width, desired_height) = framebuffer_size;
    vk::Extent2D {
        width: desired_width.clamp(
            capabilities.min_image_extent.width,
            capabilities.max_image_extent.width,
        ),
        height: desired_height.clamp(
            capabilities.min_image_extent.height,
            capabilities.max_image_extent.height,
        ),
    }
}

fn select_present_mode(cxt: &VulkanContext) -> Result<vk::PresentModeKHR> {
    let present_modes = unsafe {
        cxt.surface_khr
            .ext
            .get_physical_device_surface_present_modes(
                cxt.physical_device,
                cxt.surface_khr.raw,
            )?
    };
    log::trace!("Present modes for device:\n{:#?}", present_modes);
    if present_modes.contains(&vk::PresentModeKHR::MAILBOX) {
        Ok(vk::PresentModeKHR::MAILBOX)
    } else if present_modes.contains(&vk::PresentModeKHR::IMMEDIATE) {
        Ok(vk::PresentModeKHR::IMMEDIATE)
    } else {
        Ok(vk::PresentModeKHR::FIFO)
    }
}
