use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::{
        ext::debug_utils,
        vk::{self, Handle},
    },
    std::{ffi::CString, sync::Arc},
};

/// A RAII wrapper for the Vulkan Logical Device.
pub struct Device {
    debug_utils: Option<Arc<ash::ext::debug_utils::Device>>,
    pub raw: ash::Device,
    pub ash: Arc<raii::Instance>,
}

impl Device {
    pub fn new(
        instance: Arc<raii::Instance>,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
    ) -> Result<Arc<Self>> {
        let raw = unsafe {
            instance
                .raw
                .create_device(physical_device, create_info, None)?
        };

        let debug_utils = Self::create_debug_utils(&instance.raw, &raw);
        if let Some(debug_utils) = debug_utils.as_ref() {
            let name = CString::new("My Device").unwrap();
            unsafe {
                debug_utils.set_debug_utils_object_name(
                    &vk::DebugUtilsObjectNameInfoEXT {
                        object_type: vk::ObjectType::DEVICE,
                        object_handle: raw.handle().as_raw(),
                        p_object_name: name.as_ptr(),
                        ..Default::default()
                    },
                )?;
            }
        }
        Ok(Arc::new(Self {
            debug_utils,
            raw,
            ash: instance,
        }))
    }

    #[cfg(not(debug_assertions))]
    pub fn set_debug_name(
        &self,
        name_info: &vk::DebugUtilsObjectNameInfoEXT,
    ) -> Result<()> {
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub fn set_debug_name(
        &self,
        name_info: &vk::DebugUtilsObjectNameInfoEXT,
    ) -> Result<()> {
        unsafe {
            self.debug_utils
                .as_ref()
                .unwrap()
                .set_debug_utils_object_name(name_info)
                .with_context(trace!("Unable to set object debug name"))?
        }
        Ok(())
    }

    #[cfg(not(debug_assertions))]
    fn create_debug_utils(
        instance: &ash::Instance,
        device: &ash::Device,
    ) -> Option<Arc<debug_utils::Device>> {
        None
    }

    #[cfg(debug_assertions)]
    fn create_debug_utils(
        instance: &ash::Instance,
        device: &ash::Device,
    ) -> Option<Arc<debug_utils::Device>> {
        Some(Arc::new(ash::ext::debug_utils::Device::new(
            instance, device,
        )))
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { self.raw.destroy_device(None) }
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("raw", &"<raw vulkan device handle>")
            .field("ash", &self.ash)
            .finish()
    }
}

impl std::ops::Deref for Device {
    type Target = ash::Device;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
