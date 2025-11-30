use {
    crate::{graphics::vulkan::allocator::HumanizedSize, trace},
    anyhow::{Context, Result},
    ash::vk,
};

/// Contains all of the information required to allocate a block of memory.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AllocationRequirements {
    pub alignment: u64,
    pub allocation_size: u64,
    pub memory_type_index: u32,
    pub memory_property_flags: vk::MemoryPropertyFlags,
    pub memory_allocate_flags: vk::MemoryAllocateFlags,
    pub should_be_dedicated: bool,
}

impl AllocationRequirements {
    /// Determines the allocation requirements based on system properties.
    pub fn new(
        properties: &vk::PhysicalDeviceMemoryProperties,
        requirements: &vk::MemoryRequirements,
        memory_property_flags: vk::MemoryPropertyFlags,
        memory_allocate_flags: vk::MemoryAllocateFlags,
        dedicated: bool,
    ) -> Result<Self> {
        let (memory_type_index, _) = properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                let type_bits = 1 << index;
                let is_supported_type =
                    type_bits & requirements.memory_type_bits != 0;
                let is_visible_and_coherent =
                    memory_type.property_flags.contains(memory_property_flags);
                is_supported_type && is_visible_and_coherent
            })
            .with_context(trace!("Unable to find compatible memory type!"))?;
        Ok(Self {
            alignment: requirements.alignment,
            allocation_size: requirements.size,
            memory_type_index: memory_type_index as u32,
            memory_property_flags,
            memory_allocate_flags,
            should_be_dedicated: dedicated,
        })
    }

    /// Constructs the memory allocate info flags struct, containing the
    /// relevant flags for this allocation.
    pub fn memory_allocate_flags_info(
        &self,
    ) -> vk::MemoryAllocateFlagsInfo<'static> {
        vk::MemoryAllocateFlagsInfo {
            flags: self.memory_allocate_flags,
            ..Default::default()
        }
    }

    /// Constructs a compatible vkMemoryAllocateInfo struct.
    pub fn memory_allocate_info(&self) -> vk::MemoryAllocateInfo<'static> {
        vk::MemoryAllocateInfo {
            allocation_size: self.allocation_size,
            memory_type_index: self.memory_type_index,
            ..Default::default()
        }
    }
}

impl std::fmt::Debug for AllocationRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AllocationRequirements")
            .field("alignment", &self.alignment)
            .field("allocation_size", &HumanizedSize(self.allocation_size))
            .field("memory_type_index", &self.memory_type_index)
            .field("flags", &self.memory_property_flags)
            .field("should_be_dedicated", &self.should_be_dedicated)
            .finish()
    }
}
