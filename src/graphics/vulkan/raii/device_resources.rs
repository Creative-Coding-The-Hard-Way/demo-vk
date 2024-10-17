use {
    crate::graphics::vulkan::raii,
    anyhow::Result,
    ash::vk::{self, ImageViewCreateInfo},
    std::sync::Arc,
};

macro_rules! resource {
    (
        $name: ident,
        $raw_type: ty,
        $create_info_type: ty,
        $create: ident,
        $destroy: ident
    ) => {
        /// RAII wrapper that destroys itself when Dropped.
        ///
        /// The owner is responsible for dropping Vulkan resources in the
        /// correct order.
        pub struct $name {
            pub raw: $raw_type,
            pub device: Arc<raii::Device>,
        }

        impl $name {
            pub fn new(
                device: Arc<raii::Device>,
                create_info: &$create_info_type,
            ) -> Result<Self> {
                let raw = unsafe { device.$create(&create_info, None)? };
                Ok(Self { device, raw })
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("raw", &self.raw)
                    .field("device", &self.device)
                    .finish()
            }
        }

        impl std::ops::Deref for $name {
            type Target = $raw_type;

            fn deref(&self) -> &Self::Target {
                &self.raw
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { self.device.$destroy(self.raw, None) }
            }
        }
    };
}

resource!(
    ImageView,
    vk::ImageView,
    ImageViewCreateInfo,
    create_image_view,
    destroy_image_view
);
