use {
    anyhow::Result,
    ash::vk,
    demo_vk::{
        graphics::vulkan::{raii, spirv_words, VulkanContext},
        unwrap_here,
    },
};

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
struct PushConstants {
    pub resolution: [u32; 2],
}

pub struct Compute {
    descriptor_set: vk::DescriptorSet,
    descriptor_pool: raii::DescriptorPool,
    descriptor_set_layout: raii::DescriptorSetLayout,
    pipeline_layout: raii::PipelineLayout,
    pipeline: raii::Pipeline,
}

impl Compute {
    pub fn new(ctx: &VulkanContext) -> Result<Self> {
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
                size: std::mem::size_of::<PushConstants>() as u32,
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
            let kernel_words = unwrap_here!(
                "Include shader SPIR-V bytes.",
                spirv_words(include_bytes!("./image.comp.spv"))
            );
            let module = unwrap_here!(
                "Create shader module",
                raii::ShaderModule::new(
                    "Compute module",
                    ctx.device.clone(),
                    &vk::ShaderModuleCreateInfo {
                        code_size: kernel_words.len() * 4,
                        p_code: kernel_words.as_ptr(),
                        ..Default::default()
                    }
                )
            );
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
            descriptor_pool,
            descriptor_set_layout,
            pipeline_layout,
            pipeline,
        })
    }
}
