use {
    crate::{
        graphics::vulkan::{raii, Allocator, Block},
        unwrap_here,
    },
    anyhow::Result,
    ash::vk,
    std::sync::Arc,
};

/// A block allocation that frees itself when dropped.
#[derive(Debug)]
pub struct OwnedBlock {
    block: Block,
    allocator: Arc<Allocator>,
}

impl OwnedBlock {
    /// Creates an image and allocates memory to back it.
    ///
    /// The image is bound to the memory prior to return, so the caller can use
    /// it right away.
    pub fn allocate_image(
        allocator: Arc<Allocator>,
        image_create_info: &vk::ImageCreateInfo,
        memory_property_flags: vk::MemoryPropertyFlags,
    ) -> Result<(Self, raii::Image)> {
        let image = unwrap_here!(
            "Create Vulkan image",
            raii::Image::new(
                "Allocated Image",
                allocator.logical_device.clone(),
                image_create_info,
            )
        );

        let (requirements, dedicated) = {
            let mut dedicated = vk::MemoryDedicatedRequirements::default();
            let requirements = unsafe {
                let mut out = vk::MemoryRequirements2::default()
                    .push_next(&mut dedicated);

                allocator.logical_device.get_image_memory_requirements2(
                    &vk::ImageMemoryRequirementsInfo2 {
                        image: image.raw,
                        ..Default::default()
                    },
                    &mut out,
                );

                out.memory_requirements
            };
            (
                requirements,
                dedicated.prefers_dedicated_allocation == vk::TRUE
                    || dedicated.requires_dedicated_allocation == vk::TRUE,
            )
        };

        let block = unwrap_here!(
            "Allocate memory for Vulkan image",
            allocator.allocate_memory(
                &requirements,
                memory_property_flags,
                vk::MemoryAllocateFlags::empty(),
                dedicated,
            )
        );

        unwrap_here!("Bind memory to Vulkan image", unsafe {
            allocator.logical_device.bind_image_memory(
                image.raw,
                block.memory(),
                block.offset(),
            )
        });

        Ok((Self { block, allocator }, image))
    }

    /// Creates a buffer and allocates memory to back it.
    ///
    /// The buffer is bound to the memory prior to return, so the caller can use
    /// it right away.
    pub fn allocate_buffer(
        allocator: Arc<Allocator>,
        buffer_create_info: &vk::BufferCreateInfo,
        memory_property_flags: vk::MemoryPropertyFlags,
    ) -> Result<(OwnedBlock, raii::Buffer)> {
        let buffer = unwrap_here!(
            "Create Vulkan buffer",
            raii::Buffer::new(
                "Allocated Buffer",
                allocator.logical_device.clone(),
                buffer_create_info,
            )
        );

        let (requirements, dedicated) = {
            let mut dedicated = vk::MemoryDedicatedRequirements::default();
            let requirements = unsafe {
                let mut out = vk::MemoryRequirements2::default()
                    .push_next(&mut dedicated);

                allocator.logical_device.get_buffer_memory_requirements2(
                    &vk::BufferMemoryRequirementsInfo2 {
                        buffer: buffer.raw,
                        ..Default::default()
                    },
                    &mut out,
                );

                out.memory_requirements
            };
            (
                requirements,
                dedicated.prefers_dedicated_allocation == vk::TRUE
                    || dedicated.requires_dedicated_allocation == vk::TRUE,
            )
        };

        let memory_allocate_flags = if buffer_create_info
            .usage
            .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            vk::MemoryAllocateFlags::DEVICE_ADDRESS
        } else {
            vk::MemoryAllocateFlags::empty()
        };

        let block = unwrap_here!(
            "Allocate memory for Vulkan buffer",
            allocator.allocate_memory(
                &requirements,
                memory_property_flags,
                memory_allocate_flags,
                dedicated,
            )
        );

        unwrap_here!("Bind memory to Vulkan buffer", unsafe {
            allocator.logical_device.bind_buffer_memory(
                buffer.raw,
                block.memory(),
                block.offset(),
            )
        });

        Ok((Self { block, allocator }, buffer))
    }
}

impl std::ops::Deref for OwnedBlock {
    type Target = Block;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl Drop for OwnedBlock {
    fn drop(&mut self) {
        self.allocator.free(&self.block);
    }
}
