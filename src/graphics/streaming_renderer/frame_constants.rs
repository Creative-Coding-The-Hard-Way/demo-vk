use {
    crate::graphics::vulkan::{
        raii, Frame, FramesInFlight, UniformBuffer, VulkanContext,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// Manages all resources required to provide frame-constant data to the shader
/// pipeline.
///
/// Frame-constant means data which is specified once at the beginning of the
/// frame which does not change until the next frame. Data is stored in a CPU
/// accessible uniform buffer, as such it is optimized for data which typically
/// changes on each frame.
pub struct FrameConstants<UserDataT: Copy> {
    /// One descriptor set for each frame-in-flight.
    descriptor_sets: Vec<vk::DescriptorSet>,

    /// One descriptor pool to allocate all sets from.
    _descriptor_pool: raii::DescriptorPool,

    /// The layout for each frame's descriptor set.
    descriptor_set_layout: raii::DescriptorSetLayout,

    /// A CPU accessible buffer that has space for per-frame data.
    uniform_buffer: UniformBuffer<UserDataT>,
}

impl<UserDataT: Copy> FrameConstants<UserDataT> {
    /// Creates a new instance.
    pub fn new(
        ctx: &VulkanContext,
        frames_in_flight: &FramesInFlight,
    ) -> Result<Self> {
        let frame_count = frames_in_flight.frame_count() as u32;

        let descriptor_set_layout = {
            let bindings = [vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::VERTEX
                    | vk::ShaderStageFlags::FRAGMENT,
                p_immutable_samplers: std::ptr::null(),
                ..Default::default()
            }];
            raii::DescriptorSetLayout::new(
                "FirstTriangleDescLayout",
                ctx.device.clone(),
                &vk::DescriptorSetLayoutCreateInfo {
                    binding_count: bindings.len() as u32,
                    p_bindings: bindings.as_ptr(),
                    ..Default::default()
                },
            )
            .context("Unable to create FrameData descriptor set layout")?
        };

        let descriptor_pool = {
            let pool_sizes = [vk::DescriptorPoolSize {
                ty: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: frame_count,
            }];
            raii::DescriptorPool::new(
                "FrameData DescriptorPool",
                ctx.device.clone(),
                &vk::DescriptorPoolCreateInfo {
                    max_sets: frame_count,
                    pool_size_count: pool_sizes.len() as u32,
                    p_pool_sizes: pool_sizes.as_ptr(),
                    ..Default::default()
                },
            )
            .context("Unable to construct FrameData descriptor pool!")?
        };

        let descriptor_sets = {
            let layouts = (0..frames_in_flight.frame_count())
                .map(|_| descriptor_set_layout.raw)
                .collect::<Vec<_>>();
            unsafe {
                ctx.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo {
                    descriptor_pool: descriptor_pool.raw,
                    descriptor_set_count: layouts.len() as u32,
                    p_set_layouts: layouts.as_ptr(),
                    ..Default::default()
                })
                .context("Error while allocating descriptor sets")?
            }
        };

        let uniform_buffer =
            UniformBuffer::allocate_per_frame(ctx, frames_in_flight)
                .context("Unable to allocate UniformBuffer for FrameData")?;

        Self::write_descriptor_sets(ctx, &descriptor_sets, &uniform_buffer);

        Ok(Self {
            descriptor_sets,
            _descriptor_pool: descriptor_pool,
            descriptor_set_layout,
            uniform_buffer,
        })
    }

    pub fn set_data(&mut self, frame: &Frame, data: UserDataT) -> Result<()> {
        self.uniform_buffer.update_frame_data(frame, data)
    }

    pub fn descriptor_set_for_frame(&self, frame: &Frame) -> vk::DescriptorSet {
        self.descriptor_sets[frame.frame_index()]
    }

    pub fn descriptor_set_layout(&self) -> &raii::DescriptorSetLayout {
        &self.descriptor_set_layout
    }

    fn write_descriptor_sets(
        ctx: &VulkanContext,
        descriptor_sets: &[vk::DescriptorSet],
        uniform_buffer: &UniformBuffer<UserDataT>,
    ) {
        if std::mem::size_of::<UserDataT>() == 0 {
            // nothing to do if there is no UserData
            return;
        }

        let uniform_buffer_infos: Vec<vk::DescriptorBufferInfo> =
            descriptor_sets
                .iter()
                .enumerate()
                .map(|(index, _descriptor_set)| vk::DescriptorBufferInfo {
                    buffer: uniform_buffer.buffer(),
                    offset: uniform_buffer.offset_for_index(index),
                    range: std::mem::size_of::<UserDataT>() as u64,
                })
                .collect();
        let writes: Vec<vk::WriteDescriptorSet> = descriptor_sets
            .iter()
            .enumerate()
            .flat_map(|(index, descriptor_set)| {
                [vk::WriteDescriptorSet {
                    dst_set: *descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    p_image_info: std::ptr::null(),
                    p_buffer_info: &uniform_buffer_infos[index],
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                }]
            })
            .collect();
        unsafe { ctx.update_descriptor_sets(&writes, &[]) };
    }
}
