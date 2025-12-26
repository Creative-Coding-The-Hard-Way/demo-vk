use {crate::unwrap_here, anyhow::Result, ash::vk, std::sync::Arc};

/// A RAII wrapper for the ash library entry and instance that destroys itself
/// when dropped.
pub struct Instance {
    pub entry: ash::Entry,
    pub raw: ash::Instance,
}

impl Instance {
    pub fn new(create_info: &vk::InstanceCreateInfo) -> Result<Arc<Self>> {
        let entry = unwrap_here!("Create the Vulkan loader", unsafe {
            ash::Entry::load()
        });
        let raw = unwrap_here!("Create the Vulkan library instance", unsafe {
            entry.create_instance(create_info, None)
        });
        Ok(Arc::new(Self { entry, raw }))
    }
}

impl std::ops::Deref for Instance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.raw.destroy_instance(None);
        }
    }
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("<Ash Library Instance>").finish()
    }
}
