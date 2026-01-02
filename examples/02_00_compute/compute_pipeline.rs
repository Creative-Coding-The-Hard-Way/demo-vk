use {
    anyhow::Result,
    ash::vk,
    demo_vk::{
        graphics::{
            streaming_renderer::Texture,
            vulkan::{raii, VulkanContext},
        },
        unwrap_here,
    },
};

pub struct Compute {
    descriptor_set: vk::DescriptorSet,
    _descriptor_pool: raii::DescriptorPool,
    _descriptor_set_layout: raii::DescriptorSetLayout,
    pipeline_layout: raii::PipelineLayout,
    pipeline: raii::Pipeline,
}

impl Compute {
    pub fn new(
        ctx: &VulkanContext,
        module: &raii::ShaderModule,
    ) -> Result<Self> {
        let descriptor_set_layout = {
            let bindings = [vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::STORAGE_IMAGE,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::COMPUTE,
                p_immutable_samplers: std::ptr::null(),
                ..Default::default()
            }];
            unwrap_here!(
                "Create Compute pipeline descriptor set layout",
                raii::DescriptorSetLayout::new(
                    "Compute Pipeline layout",
                    ctx.device.clone(),
                    &vk::DescriptorSetLayoutCreateInfo {
                        binding_count: bindings.len() as u32,
                        p_bindings: bindings.as_ptr(),
                        ..Default::default()
                    }
                )
            )
        };

        let descriptor_pool = {
            let pool_sizes = [vk::DescriptorPoolSize {
                ty: vk::DescriptorType::STORAGE_IMAGE,
                descriptor_count: 1,
            }];
            unwrap_here!(
                "Create Compute descriptor pool",
                raii::DescriptorPool::new(
                    "Compute Descriptor Pool",
                    ctx.device.clone(),
                    &vk::DescriptorPoolCreateInfo {
                        max_sets: 1,
                        pool_size_count: pool_sizes.len() as u32,
                        p_pool_sizes: pool_sizes.as_ptr(),
                        ..Default::default()
                    }
                )
            )
        };

        let descriptor_set = unsafe {
            unwrap_here!(
                "Allocate descriptor set",
                ctx.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo {
                    descriptor_pool: descriptor_pool.raw,
                    descriptor_set_count: 1,
                    p_set_layouts: &descriptor_set_layout.raw,
                    ..Default::default()
                })?
                .first()
                .copied()
                .context("Expected exactly one descriptor set")
            )
        };

        let pipeline_layout = {
            let descriptor_set_layouts = [descriptor_set_layout.raw];
            let push_constant_ranges = [vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlags::COMPUTE,
                offset: 0,
                size: (std::mem::size_of::<u32>() * 2) as u32,
            }];
            unwrap_here!(
                "Compute pipeline layout",
                raii::PipelineLayout::new(
                    "Compute Pipeline Layout",
                    ctx.device.clone(),
                    &vk::PipelineLayoutCreateInfo {
                        set_layout_count: descriptor_set_layouts.len() as u32,
                        p_set_layouts: descriptor_set_layouts.as_ptr(),
                        push_constant_range_count: push_constant_ranges.len()
                            as u32,
                        p_push_constant_ranges: push_constant_ranges.as_ptr(),
                        ..Default::default()
                    }
                )
            )
        };

        let pipeline = {
            let map_entries = [
                vk::SpecializationMapEntry {
                    constant_id: 0,
                    offset: 0,
                    size: std::mem::size_of::<u32>(),
                },
                vk::SpecializationMapEntry {
                    constant_id: 1,
                    offset: std::mem::size_of::<u32>() as u32,
                    size: std::mem::size_of::<u32>(),
                },
            ];
            let data: [u32; _] = [8, 1];
            let specialization_info = vk::SpecializationInfo {
                map_entry_count: map_entries.len() as u32,
                p_map_entries: map_entries.as_ptr(),
                data_size: std::mem::size_of_val(&data),
                p_data: data.as_ptr() as _,
                ..Default::default()
            };
            unwrap_here!(
                "Create compute pipeline",
                raii::Pipeline::new_compute_pipeline(
                    ctx.device.clone(),
                    &vk::ComputePipelineCreateInfo {
                        stage: vk::PipelineShaderStageCreateInfo {
                            stage: vk::ShaderStageFlags::COMPUTE,
                            module: module.raw,
                            p_name: c"main".as_ptr(),
                            p_specialization_info: &specialization_info,
                            ..Default::default()
                        },
                        layout: pipeline_layout.raw,
                        ..Default::default()
                    }
                )
            )
        };

        Ok(Self {
            descriptor_set,
            _descriptor_pool: descriptor_pool,
            _descriptor_set_layout: descriptor_set_layout,
            pipeline_layout,
            pipeline,
        })
    }

    pub fn write_descriptor_set(&self, ctx: &VulkanContext, image: &Texture) {
        let descriptor_image_info = vk::DescriptorImageInfo {
            sampler: vk::Sampler::null(),
            image_view: image.view().raw,
            image_layout: vk::ImageLayout::GENERAL,
        };
        unsafe {
            ctx.update_descriptor_sets(
                &[vk::WriteDescriptorSet {
                    dst_set: self.descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::STORAGE_IMAGE,
                    p_image_info: &descriptor_image_info,
                    p_buffer_info: std::ptr::null(),
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                }],
                &[],
            )
        };
    }

    pub fn dispatch(
        &self,
        ctx: &VulkanContext,
        command_buffer: vk::CommandBuffer,
        image: &Texture,
    ) {
        unsafe {
            ctx.cmd_push_constants(
                command_buffer,
                self.pipeline_layout.raw,
                vk::ShaderStageFlags::COMPUTE,
                0,
                &image.width().to_le_bytes(),
            );
            ctx.cmd_push_constants(
                command_buffer,
                self.pipeline_layout.raw,
                vk::ShaderStageFlags::COMPUTE,
                4,
                &image.height().to_le_bytes(),
            );
            ctx.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline_layout.raw,
                0,
                &[self.descriptor_set],
                &[],
            );
            ctx.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline.raw,
            );
            ctx.cmd_dispatch(
                command_buffer,
                image.width() / 8,
                image.height() / 8,
                1,
            );
        }
    }
}
