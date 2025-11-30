use {
    super::ComposableAllocator,
    crate::graphics::vulkan::{allocator::AllocationRequirements, Block},
    anyhow::Result,
    ash::vk,
    std::collections::{hash_map::Entry, HashMap},
};

/// Organizes allocation requests by the memory type index.
///
/// Allocations from the same memory type index can be broken apart and
/// suballocated, etc...
pub struct TypeIndexAllocator {
    allocators: HashMap<(u32, bool), Box<dyn ComposableAllocator>>,
    type_index_factory: Box<dyn Fn(u32, bool) -> Box<dyn ComposableAllocator>>,
}

impl TypeIndexAllocator {
    pub fn new<T, F>(type_index_factory: F) -> Self
    where
        T: ComposableAllocator + 'static,
        F: Fn(u32, bool) -> T + 'static,
    {
        let type_index_factory =
            move |index, addressable| -> Box<dyn ComposableAllocator> {
                let alloc = type_index_factory(index, addressable);
                Box::new(alloc)
            };
        Self {
            allocators: HashMap::with_capacity(8),
            type_index_factory: Box::new(type_index_factory),
        }
    }
}

impl ComposableAllocator for TypeIndexAllocator {
    fn owns(&self, block: &Block) -> bool {
        self.allocators
            .values()
            .any(|allocator| allocator.owns(block))
    }

    fn allocate_memory(
        &mut self,
        requirements: AllocationRequirements,
    ) -> Result<Block> {
        let addressable = requirements
            .memory_allocate_flags
            .contains(vk::MemoryAllocateFlags::DEVICE_ADDRESS);
        if let Entry::Vacant(e) = self
            .allocators
            .entry((requirements.memory_type_index, addressable))
        {
            e.insert((self.type_index_factory)(
                requirements.memory_type_index,
                addressable,
            ));
        }

        self.allocators
            .get_mut(&(requirements.memory_type_index, addressable))
            .unwrap()
            .allocate_memory(requirements)
    }

    fn free_memory(&mut self, block: &Block) {
        self.allocators
            .get_mut(&(
                block.memory_type_index(),
                block.is_device_addressable(),
            ))
            .expect("Invalid block memory type index!")
            .free_memory(block);
    }
}
