mod debug;

use {
    crate::{graphics::vulkan::raii, unwrap_here},
    anyhow::{Context, Result},
    ash::vk,
    std::{ffi::CStr, sync::Arc},
    winit::{raw_window_handle::HasDisplayHandle, window::Window},
};

const VK_EXT_DEBUG_UTILS: &CStr = c"VK_EXT_debug_utils";

/// The logical Vulkan instance.
///
/// The instance contains the ash library entry, instance, and any associated
/// debugging information. Within the scope of this library, the Instance is
/// expected to outlive all other Vulkan resources e.g. it should only be
/// dropped once all other resources have been destroyed or dropped.
pub struct Instance {
    pub ash: Arc<raii::Instance>,
    extensions: Vec<String>,
    _debug_utils: Option<Arc<raii::DebugUtils>>,
}

impl Instance {
    /// Create a new Vulkan instance for the given GLFW window.
    pub fn for_window(
        app_name: impl AsRef<str>,
        window: &Window,
    ) -> Result<Self> {
        let extensions = ash_window::enumerate_required_extensions(
            window
                .display_handle()
                .with_context(|| "Unable to fetch display handle!")?
                .as_raw(),
        )?;

        Self::new(app_name, &extensions)
    }

    /// Create a new Vulkan instance.
    pub fn new(
        app_name: impl AsRef<str>,
        extensions: &[*const i8],
    ) -> Result<Self> {
        let ptrs = {
            let mut ptrs = extensions.to_vec();
            if cfg!(debug_assertions) {
                ptrs.push(VK_EXT_DEBUG_UTILS.as_ptr());
            }
            ptrs
        };

        let app_name_c = std::ffi::CString::new(app_name.as_ref()).unwrap();
        let engine_name = std::ffi::CString::new("N/A").unwrap();
        let application_info = vk::ApplicationInfo {
            p_application_name: app_name_c.as_ptr(),
            application_version: vk::make_api_version(0, 1, 0, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            api_version: vk::make_api_version(0, 1, 3, 0),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &application_info,
            enabled_extension_count: ptrs.len() as u32,
            pp_enabled_extension_names: ptrs.as_ptr(),
            ..Default::default()
        };

        let ash = unwrap_here!(
            "Create Vulkan instance",
            raii::Instance::new(&create_info)
        );

        let debug_utils = unwrap_here!(
            "Setup debug logging.",
            debug::setup_debug_logging(ash.clone())
        );

        Ok(Self {
            ash,
            extensions:
        ptrs.iter().map(|&ptr| {
            unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string()
        }).collect(),
            _debug_utils: debug_utils,
        })
    }
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("extensions", &self.extensions)
            .field("ash", &self.ash)
            .finish()
    }
}

impl std::ops::Deref for Instance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        &self.ash.raw
    }
}
