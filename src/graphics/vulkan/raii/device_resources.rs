use {
    crate::{graphics::vulkan::raii, trace},
    anyhow::{Context, Result},
    ash::vk::{self, Handle},
    std::{ffi::CString, sync::Arc},
};

macro_rules! resource {
    (
        $name: ident,
        $raw_type: ty,
        $object_type: expr,
        $create_info_type: ty,
        $create: ident,
        $destroy: ident
    ) => {
        resource_constructor!(
            $name,
            $raw_type,
            $create_info_type,
            $create,
            $destroy
        );
        resource_impl!($name, $raw_type, $object_type, $destroy);
    };
}

macro_rules! resource_constructor {
    (
        $name: ident,
        $raw_type: ty,
        $create_info_type: ty,
        $create: ident,
        $destroy: ident
    ) => {
        impl $name {
            #[doc = "Creates a new instance."]
            pub fn new(
                name: impl Into<String>,
                device: Arc<raii::Device>,
                create_info: &$create_info_type,
            ) -> Result<Self> {
                let raw = unsafe { device.$create(&create_info, None)? };
                let instance = Self { device, raw };
                instance.set_debug_name(name)?;
                Ok(instance)
            }
        }
    };
}

macro_rules! resource_impl {
    (
        $name: ident,
        $raw_type: ty,
        $object_type: expr,
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
            #[doc = "Sets the debug name used for validation layer logging."]
            pub fn set_debug_name(
                &self,
                name: impl Into<String>,
            ) -> Result<()> {
                let object_type = $object_type;
                let name =
                    CString::new(format!("{:?}: {}", object_type, name.into()))
                        .unwrap();

                self.device
                    .set_debug_name(&vk::DebugUtilsObjectNameInfoEXT {
                        object_type,
                        object_handle: self.raw.as_raw(),
                        p_object_name: name.as_ptr(),
                        ..Default::default()
                    })
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
    Sampler,
    vk::Sampler,
    vk::Sampler::TYPE,
    vk::SamplerCreateInfo,
    create_sampler,
    destroy_sampler
);

resource!(
    Image,
    vk::Image,
    vk::Image::TYPE,
    vk::ImageCreateInfo,
    create_image,
    destroy_image
);

resource!(
    Fence,
    vk::Fence,
    vk::Fence::TYPE,
    vk::FenceCreateInfo,
    create_fence,
    destroy_fence
);

resource!(
    DeviceMemory,
    vk::DeviceMemory,
    vk::DeviceMemory::TYPE,
    vk::MemoryAllocateInfo,
    allocate_memory,
    free_memory
);

resource!(
    Buffer,
    vk::Buffer,
    vk::Buffer::TYPE,
    vk::BufferCreateInfo,
    create_buffer,
    destroy_buffer
);

resource!(
    DescriptorPool,
    vk::DescriptorPool,
    vk::DescriptorPool::TYPE,
    vk::DescriptorPoolCreateInfo,
    create_descriptor_pool,
    destroy_descriptor_pool
);

resource!(
    DescriptorSetLayout,
    vk::DescriptorSetLayout,
    vk::DescriptorSetLayout::TYPE,
    vk::DescriptorSetLayoutCreateInfo,
    create_descriptor_set_layout,
    destroy_descriptor_set_layout
);

resource!(
    ImageView,
    vk::ImageView,
    vk::ImageView::TYPE,
    vk::ImageViewCreateInfo,
    create_image_view,
    destroy_image_view
);

resource!(
    Semaphore,
    vk::Semaphore,
    vk::Semaphore::TYPE,
    vk::SemaphoreCreateInfo,
    create_semaphore,
    destroy_semaphore
);

resource!(
    CommandPool,
    vk::CommandPool,
    vk::CommandPool::TYPE,
    vk::CommandPoolCreateInfo,
    create_command_pool,
    destroy_command_pool
);

resource!(
    RenderPass,
    vk::RenderPass,
    vk::RenderPass::TYPE,
    vk::RenderPassCreateInfo,
    create_render_pass,
    destroy_render_pass
);

resource!(
    Framebuffer,
    vk::Framebuffer,
    vk::Framebuffer::TYPE,
    vk::FramebufferCreateInfo,
    create_framebuffer,
    destroy_framebuffer
);

resource!(
    ShaderModule,
    vk::ShaderModule,
    vk::ShaderModule::TYPE,
    vk::ShaderModuleCreateInfo,
    create_shader_module,
    destroy_shader_module
);

resource!(
    PipelineLayout,
    vk::PipelineLayout,
    vk::PipelineLayout::TYPE,
    vk::PipelineLayoutCreateInfo,
    create_pipeline_layout,
    destroy_pipeline_layout
);

// Pipeline is a special case because there are separate create infos for each
// kind of pipeline.
resource_impl!(Pipeline, vk::Pipeline, vk::Pipeline::TYPE, destroy_pipeline);

impl Pipeline {
    pub fn new_graphics_pipeline(
        device: Arc<raii::Device>,
        create_info: &vk::GraphicsPipelineCreateInfo,
    ) -> Result<Self> {
        let result = unsafe {
            device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[*create_info],
                None,
            )
        };
        let raw = match result {
            Ok(pipelines) => pipelines[0],
            Err((_, result)) => {
                return Err(result).with_context(trace!(
                    "Error while creating graphics pipeline!"
                ));
            }
        };
        Ok(Self { device, raw })
    }

    pub fn new_compute_pipeline(
        device: Arc<raii::Device>,
        create_info: &vk::ComputePipelineCreateInfo,
    ) -> Result<Self> {
        let result = unsafe {
            device.create_compute_pipelines(
                vk::PipelineCache::null(),
                &[*create_info],
                None,
            )
        };
        let raw = match result {
            Ok(pipelines) => pipelines[0],
            Err((_, result)) => {
                return Err(result).with_context(trace!(
                    "Error while creating compute pipeline!"
                ));
            }
        };
        Ok(Self { device, raw })
    }
}
