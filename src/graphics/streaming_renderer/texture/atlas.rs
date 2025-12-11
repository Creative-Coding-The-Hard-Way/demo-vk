use {
    super::Texture,
    crate::graphics::vulkan::{raii, VulkanContext},
    anyhow::{Context, Result},
    ash::vk,
    std::sync::Arc,
};

/// Used to constrain the variable descriptor array and pool sizes. This is well
/// below the 500_000 limit imposed by the spec for the minimum supported number
/// of UPDATE_AFTER_BIND descriptors in a descriptor poolw.
const MAX_TEXTURES: u32 = 10_000;

/// The TextureAtlas maintains the per-frame descriptor sets used for binding
/// textures.
///
/// New textures can be added to the atlas at any time and will be available for
/// use in the next frame.
///
/// # Shader Requirements
///
/// To use the texture atlas, the fragment shader must include a descriptor set
/// with a sampler binding and a texture binding. This looks something like this
/// in glsl:
///
/// ```glsl
/// layout(set = 0, binding = 0) uniform sampler u_Sampler;
/// layout(set = 0, binding = 1) uniform texture2D u_Textures[];
/// ```
pub struct TextureAtlas {
    // All textures currently in-use by the atlas.
    textures: Vec<Arc<Texture>>,

    // The sampler used when reading textures.
    _sampler: raii::Sampler,

    descriptor_set_layout: raii::DescriptorSetLayout,
    descriptor_set: vk::DescriptorSet,
    _descriptor_pool: raii::DescriptorPool,
}

impl TextureAtlas {
    pub fn new(ctx: &VulkanContext) -> Result<Self> {
        let sampler = raii::Sampler::new(
            "TextureAtlas Immutable Sampler",
            ctx.device.clone(),
            &vk::SamplerCreateInfo {
                mag_filter: vk::Filter::LINEAR,
                min_filter: vk::Filter::LINEAR,
                mipmap_mode: vk::SamplerMipmapMode::LINEAR,
                address_mode_u: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                address_mode_v: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                address_mode_w: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                mip_lod_bias: 0.0,
                anisotropy_enable: vk::FALSE,
                max_anisotropy: 0.0,
                compare_enable: vk::FALSE,
                compare_op: vk::CompareOp::ALWAYS,
                min_lod: 0.0,
                max_lod: vk::LOD_CLAMP_NONE,
                border_color: vk::BorderColor::FLOAT_OPAQUE_WHITE,
                unnormalized_coordinates: vk::FALSE,
                ..Default::default()
            },
        )
        .context("Unable to create texture atlas sampler!")?;

        let descriptor_set_layout = {
            let bindings = [
                vk::DescriptorSetLayoutBinding {
                    binding: 0,
                    descriptor_type: vk::DescriptorType::SAMPLER,
                    descriptor_count: 1,
                    stage_flags: vk::ShaderStageFlags::FRAGMENT,
                    p_immutable_samplers: &sampler.raw,
                    ..Default::default()
                },
                vk::DescriptorSetLayoutBinding {
                    binding: 1,
                    descriptor_type: vk::DescriptorType::SAMPLED_IMAGE,
                    descriptor_count: MAX_TEXTURES,
                    stage_flags: vk::ShaderStageFlags::FRAGMENT,
                    p_immutable_samplers: std::ptr::null(),
                    _marker: std::marker::PhantomData,
                },
            ];
            let binding_flags = [
                vk::DescriptorBindingFlags::empty(),
                vk::DescriptorBindingFlags::PARTIALLY_BOUND
                    | vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT
                    | vk::DescriptorBindingFlags::UPDATE_AFTER_BIND
                    | vk::DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING,
            ];
            let mut binding_flags_create_info =
                vk::DescriptorSetLayoutBindingFlagsCreateInfo {
                    binding_count: binding_flags.len() as u32,
                    p_binding_flags: binding_flags.as_ptr(),
                    ..Default::default()
                };
            let create_info = vk::DescriptorSetLayoutCreateInfo {
                flags:
                    vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL,
                binding_count: bindings.len() as u32,
                p_bindings: bindings.as_ptr(),
                ..Default::default()
            }
            .push_next(&mut binding_flags_create_info);
            raii::DescriptorSetLayout::new(
                "TextureAtlas Descriptor Set layout",
                ctx.device.clone(),
                &create_info,
            )
            .context("Unable to create TextureAtlas descriptor set layout!")?
        };
        let descriptor_pool = {
            let pool_sizes = [
                vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::SAMPLER,
                    descriptor_count: 1,
                },
                vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::SAMPLED_IMAGE,
                    descriptor_count: MAX_TEXTURES,
                },
            ];
            raii::DescriptorPool::new(
                "TextureAtlas DescriptorPool",
                ctx.device.clone(),
                &vk::DescriptorPoolCreateInfo {
                    flags: vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND,
                    max_sets: 1,
                    pool_size_count: pool_sizes.len() as u32,
                    p_pool_sizes: pool_sizes.as_ptr(),
                    ..Default::default()
                },
            )
            .context("Error creating TextureAtlas descriptor pool!")?
        };
        let descriptor_set = {
            let counts = [MAX_TEXTURES];
            let mut descriptor_set_variable_descriptor_count_allocate_info =
                vk::DescriptorSetVariableDescriptorCountAllocateInfo {
                    descriptor_set_count: 1,
                    p_descriptor_counts: counts.as_ptr(),
                    ..Default::default()
                };
            let allocate_info = vk::DescriptorSetAllocateInfo {
                descriptor_pool: descriptor_pool.raw,
                descriptor_set_count: 1,
                p_set_layouts: &descriptor_set_layout.raw,
                ..Default::default()
            }
            .push_next(
                &mut descriptor_set_variable_descriptor_count_allocate_info,
            );
            unsafe {
                ctx.allocate_descriptor_sets(&allocate_info).context(
                    "Unable to allocate TextureAtlas descriptor set!",
                )?[0]
            }
        };

        Ok(Self {
            textures: vec![],
            _sampler: sampler,
            descriptor_set_layout,
            _descriptor_pool: descriptor_pool,
            descriptor_set,
        })
    }

    pub fn descriptor_set_layout(&self) -> &raii::DescriptorSetLayout {
        &self.descriptor_set_layout
    }

    /// Returns the atlas's descriptor set.
    ///
    /// Used by the application to bind the atlas descriptor.
    pub fn descriptor_set(&self) -> vk::DescriptorSet {
        self.descriptor_set
    }

    /// Adds a texture to the atlas and returns the index which can be used in
    /// vertices to reference the texture again.
    pub fn add_texture(
        &mut self,
        ctx: &VulkanContext,
        texture: Arc<Texture>,
    ) -> i32 {
        let texture_index = self.textures.len() as i32;
        let view = texture.view().raw;
        self.textures.push(texture);

        // Safe because this only writes to the newest texture index, which was
        // never in use until after this method returns it.
        unsafe {
            let image_info = vk::DescriptorImageInfo {
                sampler: vk::Sampler::null(),
                image_view: view,
                image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            };
            ctx.update_descriptor_sets(
                &[vk::WriteDescriptorSet {
                    dst_set: self.descriptor_set,
                    dst_binding: 1,
                    dst_array_element: texture_index as u32,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::SAMPLED_IMAGE,
                    p_image_info: &image_info,
                    p_buffer_info: std::ptr::null(),
                    p_texel_buffer_view: std::ptr::null(),
                    ..Default::default()
                }],
                &[],
            );
        }

        texture_index
    }

    /// Fetchs a texture by the id returned by [Self::add_texture].
    pub fn get_texture(&self, texture_id: i32) -> &Arc<Texture> {
        &self.textures[texture_id as usize]
    }
}
