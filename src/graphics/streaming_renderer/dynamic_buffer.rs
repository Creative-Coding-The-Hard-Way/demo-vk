use {
    super::utility::round_to_power_of_two,
    crate::graphics::vulkan::{CPUBuffer, VulkanContext},
    anyhow::{Context, Result},
    ash::vk,
};

/// An automatically resizable CPU buffer that reallocates the underlying buffer
/// if needed.
pub struct DynamicBuffer<DataT: Copy> {
    usage: vk::BufferUsageFlags,
    cpu_buffer: CPUBuffer<DataT>,
    buffer_device_address: vk::DeviceAddress,
}

impl<DataT: Copy> DynamicBuffer<DataT> {
    pub fn new(
        ctx: &VulkanContext,
        initial_capacity: usize,
        usage: vk::BufferUsageFlags,
    ) -> Result<Self> {
        let cpu_buffer = CPUBuffer::allocate(
            ctx,
            round_to_power_of_two(initial_capacity),
            usage,
        )?;

        let address = if usage
            .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            unsafe {
                ctx.get_buffer_device_address(&vk::BufferDeviceAddressInfo {
                    buffer: cpu_buffer.buffer(),
                    ..Default::default()
                })
            }
        } else {
            0
        };

        Ok(Self {
            usage,
            cpu_buffer,
            buffer_device_address: address,
        })
    }

    /// Returns the raw buffer handle.
    ///
    /// # Safety
    ///
    /// Note that the returned buffer handle can be invalidated by calls to
    /// write_data.
    pub fn raw(&self) -> vk::Buffer {
        self.cpu_buffer.buffer()
    }

    /// Returns the current buffer device address.
    ///
    /// Only valid if the buffer was created with the
    /// `vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS` flag.
    pub fn buffer_device_address(&self) -> vk::DeviceAddress {
        self.buffer_device_address
    }

    /// Writes the provided data to the underlying buffer.
    ///
    /// # Safety
    ///
    /// The caller is responsible for synchronizing access to the underlying
    /// buffer. This method can reallocate the buffer, which will cause
    /// problems if the GPU is still accessing the buffer when this function
    /// is called.
    pub unsafe fn write_chunked_data(
        &mut self,
        ctx: &VulkanContext,
        data: &[&[DataT]],
    ) -> Result<bool> {
        let reallocated = unsafe {
            self.maybe_reallocate(
                ctx,
                data.iter().map(|chunk| chunk.len()).sum(),
            )?
        };

        let mut offset = 0;
        for chunk in data {
            unsafe {
                self.cpu_buffer.write_data(offset, chunk)?;
            }
            offset += chunk.len();
        }

        Ok(reallocated)
    }

    /// Writes iterated data to the underlying buffer.
    ///
    /// Must be an exact size iterator so the buffer can be reallocated if
    /// necessary without an additional copy on the CPU.
    ///
    /// # Safety
    ///
    /// The caller is responsible for synchronizing access to the underlying
    /// buffer. This method can reallocate the buffer, which will cause
    /// problems if the GPU is still accessing the buffer when this function
    /// is called.
    pub unsafe fn write_iterated_data<I>(
        &mut self,
        ctx: &VulkanContext,
        data: I,
    ) -> Result<bool>
    where
        I: ExactSizeIterator<Item = DataT>,
    {
        let reallocated = unsafe {
            self.maybe_reallocate(
                ctx,
                data.len() * std::mem::size_of::<DataT>(),
            )?
        };

        for (index, item) in data.enumerate() {
            unsafe { self.cpu_buffer.write_data(index, &[item])? }
        }

        Ok(reallocated)
    }

    /// Checks that the buffer has enough space for `required_size` and
    /// reallocates the underlying storage if needed.
    ///
    /// # Returns
    ///
    /// Returns `true` when the buffer was reallocated and `false` if not.
    ///
    /// # Safety
    ///
    /// Unsafe because the caller must synchronize access to the buffer and
    /// ensure it is not in use by the GPU when this function is called as
    /// the buffer _could_ be deleted during reallocation.
    unsafe fn maybe_reallocate(
        &mut self,
        ctx: &VulkanContext,
        required_size: usize,
    ) -> Result<bool> {
        if self.cpu_buffer.capacity() >= required_size {
            return Ok(false);
        }

        let new_size = round_to_power_of_two(required_size);

        log::trace!(
            "Reallocating. Current size: {}, Required size: {}, New size: {}",
            self.cpu_buffer.capacity(),
            required_size,
            new_size
        );

        self.cpu_buffer = CPUBuffer::allocate(ctx, new_size, self.usage)
            .context("Unable to reallocate new buffer!")?;

        if self
            .usage
            .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            self.buffer_device_address = unsafe {
                ctx.get_buffer_device_address(&vk::BufferDeviceAddressInfo {
                    buffer: self.cpu_buffer.buffer(),
                    ..Default::default()
                })
            }
        }

        Ok(true)
    }
}
