mod atlas;
mod loader;

use {
    crate::graphics::vulkan::{raii, OwnedBlock, VulkanContext},
    anyhow::{Context, Result},
    ash::vk,
};

pub use self::{atlas::TextureAtlas, loader::TextureLoader};

/// A 2D image for use when rendering.
///
/// # Safety
///
/// Textures own their own Vulkan resources and will destroy them when dropped.
/// The application is responsible for synchronizing access to Texture
/// resources with the GPU and ensuring nothing is dropped early.
pub struct Texture {
    mip_levels: u32,
    width: u32,
    height: u32,
    image_view: raii::ImageView,
    image: raii::Image,
    block: OwnedBlock,
}

#[bon::bon]
impl Texture {
    #[builder]
    pub fn new(
        ctx: &VulkanContext,
        dimensions: (u32, u32),
        format: vk::Format,
        image_usage_flags: vk::ImageUsageFlags,
        memory_property_flags: vk::MemoryPropertyFlags,
        #[builder(default = 1)] mip_levels: u32,
    ) -> Result<Self> {
        let (width, height) = dimensions;

        let (block, image) = OwnedBlock::allocate_image(
            ctx.allocator.clone(),
            &vk::ImageCreateInfo {
                flags: vk::ImageCreateFlags::empty(),
                image_type: vk::ImageType::TYPE_2D,
                format,
                extent: vk::Extent3D {
                    width,
                    height,
                    depth: 1,
                },
                mip_levels,
                array_layers: 1,
                samples: vk::SampleCountFlags::TYPE_1,
                tiling: vk::ImageTiling::OPTIMAL,
                usage: image_usage_flags,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &ctx.graphics_queue_family_index,
                initial_layout: vk::ImageLayout::UNDEFINED,
                ..Default::default()
            },
            memory_property_flags,
        )
        .context("Unable to create texture image!")?;

        let is_depth = image_usage_flags
            .contains(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT);
        let image_view = raii::ImageView::new(
            "Texture Image View",
            ctx.device.clone(),
            &vk::ImageViewCreateInfo {
                image: image.raw,
                view_type: vk::ImageViewType::TYPE_2D,
                format,
                components: vk::ComponentMapping::default(),
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: if is_depth {
                        vk::ImageAspectFlags::DEPTH
                    } else {
                        vk::ImageAspectFlags::COLOR
                    },
                    base_mip_level: 0,
                    level_count: mip_levels,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            },
        )
        .context("Unable to create texture image view")?;

        Ok(Self {
            mip_levels,
            width,
            height,
            image_view,
            image,
            block,
        })
    }

    /// Returns the underlying Vulkan image.
    pub fn image(&self) -> &raii::Image {
        &self.image
    }

    /// Returns the underlying Vulkan image view.
    pub fn view(&self) -> &raii::ImageView {
        &self.image_view
    }

    /// Returns the underlying Vulkan memory block.
    pub fn memory(&self) -> &OwnedBlock {
        &self.block
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn extent(&self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.width,
            height: self.height,
        }
    }

    #[builder]
    pub fn pipeline_barrier(
        &self,
        ctx: &VulkanContext,
        command_buffer: vk::CommandBuffer,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
        src_access_mask: vk::AccessFlags,
        dst_access_mask: vk::AccessFlags,
        src_stage_mask: vk::PipelineStageFlags,
        dst_stage_mask: vk::PipelineStageFlags,
    ) {
        let is_depth_texture = dst_access_mask
            .contains(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ)
            || dst_access_mask
                .contains(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE);

        let image_memory_barrier = vk::ImageMemoryBarrier {
            old_layout,
            new_layout,
            src_access_mask,
            dst_access_mask,
            image: self.image.raw,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: if is_depth_texture {
                    vk::ImageAspectFlags::DEPTH
                } else {
                    vk::ImageAspectFlags::COLOR
                },
                base_mip_level: 0,
                level_count: self.mip_levels,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };
        unsafe {
            ctx.cmd_pipeline_barrier(
                command_buffer,
                src_stage_mask,
                dst_stage_mask,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }
    }
}
