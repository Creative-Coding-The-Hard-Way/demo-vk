use {
    super::{super::utility::round_to_power_of_two, Texture},
    crate::graphics::vulkan::{CPUBuffer, SyncCommands, VulkanContext},
    anyhow::{Context, Result},
    ash::vk::{self},
    image::{imageops::FilterType, DynamicImage, RgbaImage},
    std::{path::PathBuf, sync::Arc},
};

/// A utility for loading textures from image files.
pub struct TextureLoader {
    sync_commands: SyncCommands,
    transfer_buffer: CPUBuffer<u8>,
    ctx: Arc<VulkanContext>,
}

impl TextureLoader {
    /// Creates a new texture loader instance.
    pub fn new(ctx: Arc<VulkanContext>) -> Result<Self> {
        Ok(Self {
            sync_commands: SyncCommands::new(ctx.clone()).context(
                "Unable to create SyncCommands for the TextureLoader!",
            )?,
            transfer_buffer: CPUBuffer::allocate(
                &ctx,
                1024 * 1024,
                vk::BufferUsageFlags::TRANSFER_SRC,
            )
            .context("Unable to allocate transfer buffer")?,
            ctx,
        })
    }

    /// Synchronously loads a texture from an image file. Supports .png, .bmp,
    /// and .jpg files
    pub fn load_from_file(
        &mut self,
        path: impl Into<PathBuf>,
        generate_mipmaps: bool,
    ) -> Result<Texture> {
        let path: PathBuf = path.into();

        let image = image::ImageReader::open(&path)
            .context(format!("Unable to open image at {:?}", &path))?
            .decode()
            .context(format!("Unable to decode image from file {:?}", &path))?;

        let mipmaps = if generate_mipmaps {
            self.compute_generated_mipmaps(image)
        } else {
            vec![image.to_rgba8()]
        };

        let texture = Texture::builder()
            .ctx(&self.ctx)
            .dimensions(mipmaps[0].dimensions())
            .format(vk::Format::R8G8B8A8_UNORM)
            .image_usage_flags(
                vk::ImageUsageFlags::TRANSFER_DST
                    | vk::ImageUsageFlags::SAMPLED,
            )
            .memory_property_flags(vk::MemoryPropertyFlags::DEVICE_LOCAL)
            .mip_levels(mipmaps.len() as u32)
            .build()
            .context(format!(
                "Unable to create texture for image file {:?}",
                &path
            ))?;

        self.copy_mipmaps_to_transfer_buffer(&mipmaps)
            .context("Unable to upload texture data!")?;

        self.copy_transfer_buffer_mipmaps_to_device_memory(&texture, &mipmaps)
            .context("Unable to copy transfer buffer to texture memory!")?;

        Ok(texture)
    }

    pub fn tex_sub_image(
        &mut self,
        ctx: &VulkanContext,
        texture: &Texture,
        rgba_data: &[u8],
        offset: [u32; 2],
        size: [u32; 2],
    ) -> Result<()> {
        debug_assert!(
            (size[0] * size[1] * 4) as usize == rgba_data.len(),
            "RGBA data and image size do not match!"
        );

        self.maybe_resize_transfer_buffer(rgba_data.len())?;
        unsafe {
            self.transfer_buffer.write_data(0, rgba_data).context(
                "Unable to write tex_sub_image data to transfer buffer",
            )?;
        }

        self.sync_commands.submit_and_wait(|cmd| {
            texture
                .pipeline_barrier()
                .ctx(ctx)
                .command_buffer(cmd)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .src_access_mask(vk::AccessFlags::SHADER_READ)
                .src_stage_mask(vk::PipelineStageFlags::ALL_COMMANDS)
                .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags::ALL_COMMANDS)
                .call();
            unsafe {
                ctx.cmd_copy_buffer_to_image(
                    cmd,
                    self.transfer_buffer.buffer(),
                    texture.image().raw,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[vk::BufferImageCopy {
                        buffer_offset: 0,
                        buffer_row_length: 0,
                        buffer_image_height: 0,
                        image_subresource: vk::ImageSubresourceLayers {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            mip_level: 0,
                            base_array_layer: 0,
                            layer_count: 1,
                        },
                        image_offset: vk::Offset3D {
                            x: offset[0] as i32,
                            y: offset[1] as i32,
                            z: 0,
                        },
                        image_extent: vk::Extent3D {
                            width: size[0],
                            height: size[1],
                            depth: 1,
                        },
                    }],
                );
            }
            texture
                .pipeline_barrier()
                .ctx(ctx)
                .command_buffer(cmd)
                .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .src_stage_mask(vk::PipelineStageFlags::TRANSFER)
                .dst_access_mask(vk::AccessFlags::SHADER_READ)
                .dst_stage_mask(vk::PipelineStageFlags::ALL_COMMANDS)
                .call();
            Ok(())
        })?;
        Ok(())
    }
}

// Private Functions
impl TextureLoader {
    /// Rezizes the transfer buffer if needed to hold `total_size` bytes. No-op
    /// if the transfer buffer is big enough to hold `total_size` bytes
    /// already.
    fn maybe_resize_transfer_buffer(
        &mut self,
        total_size_in_bytes: usize,
    ) -> Result<()> {
        if self.transfer_buffer.size_in_bytes() as usize >= total_size_in_bytes
        {
            return Ok(());
        }

        log::trace!(
            "Existing transfer buffer {:?} needs reallocated",
            self.transfer_buffer
        );

        self.transfer_buffer = CPUBuffer::allocate(
            &self.ctx,
            round_to_power_of_two(total_size_in_bytes),
            vk::BufferUsageFlags::TRANSFER_SRC,
        )
        .context("Unable to reallocate transfer buffer")?;
        log::trace!(
            "New transfer buffer allocated: {:?}",
            self.transfer_buffer
        );
        Ok(())
    }

    /// Generates mipmaps for the image.
    fn compute_generated_mipmaps(
        &self,
        mut image: DynamicImage,
    ) -> Vec<RgbaImage> {
        let mut mipmaps = vec![image.to_rgba8()];
        let mut width = image.width();
        let mut height = image.height();
        while width > 2 && height > 2 {
            width /= 2;
            height /= 2;
            image = image.resize(width, height, FilterType::Lanczos3);
            mipmaps.push(image.to_rgba8());
        }
        mipmaps
    }

    /// Copies data to the transfer buffer.
    ///
    /// This function may reallocate a new transfer buffer if the existing one
    /// is too small for the data.
    fn copy_mipmaps_to_transfer_buffer(
        &mut self,
        mipmaps: &[RgbaImage],
    ) -> Result<()> {
        self.maybe_resize_transfer_buffer(
            mipmaps.iter().map(|image| image.as_raw().len()).sum(),
        )?;

        let mut offset = 0;
        for mipmap in mipmaps {
            // # Safety
            //
            // Safe because the transfer buffer is not shared and is only used
            // for synchronous commands sent to the GPU and because
            // the transfer buffer is always resized to have enough
            // capacity for all data.
            unsafe {
                self.transfer_buffer
                    .write_data(offset, mipmap.as_raw())
                    .context("Unable to write data to transfer buffer")?;
            }
            offset += mipmap.as_raw().len();
        }
        Ok(())
    }

    /// Copies the contents of the transfer buffer into the texture's device
    /// memory.
    fn copy_transfer_buffer_mipmaps_to_device_memory(
        &self,
        texture: &Texture,
        mipmaps: &[RgbaImage],
    ) -> Result<()> {
        let ctx = &self.ctx;
        self.sync_commands.submit_and_wait(|command_buffer| unsafe {
            ctx.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[vk::ImageMemoryBarrier {
                    src_access_mask: vk::AccessFlags::empty(),
                    dst_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                    old_layout: vk::ImageLayout::UNDEFINED,
                    new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    src_queue_family_index: ctx.graphics_queue_family_index,
                    dst_queue_family_index: ctx.graphics_queue_family_index,
                    image: texture.image().raw,
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: mipmaps.len() as u32,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    ..Default::default()
                }],
            );
            let buffer_image_copies: Vec<vk::BufferImageCopy> = {
                let mut offset: u64 = 0;
                mipmaps
                    .iter()
                    .enumerate()
                    .map(|(mip_level, mipmap)| {
                        let buffer_image_copy = vk::BufferImageCopy {
                            buffer_offset: offset,
                            buffer_row_length: 0,
                            buffer_image_height: 0,
                            image_subresource: vk::ImageSubresourceLayers {
                                aspect_mask: vk::ImageAspectFlags::COLOR,
                                mip_level: mip_level as u32,
                                base_array_layer: 0,
                                layer_count: 1,
                            },
                            image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                            image_extent: vk::Extent3D {
                                width: mipmap.width(),
                                height: mipmap.height(),
                                depth: 1,
                            },
                        };
                        offset += mipmap.as_raw().len() as u64;
                        buffer_image_copy
                    })
                    .collect()
            };
            ctx.cmd_copy_buffer_to_image(
                command_buffer,
                self.transfer_buffer.buffer(),
                texture.image().raw,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &buffer_image_copies,
            );
            ctx.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[vk::ImageMemoryBarrier {
                    src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                    dst_access_mask: vk::AccessFlags::empty(),
                    old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    src_queue_family_index: ctx.graphics_queue_family_index,
                    dst_queue_family_index: ctx.graphics_queue_family_index,
                    image: texture.image().raw,
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: mipmaps.len() as u32,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    ..Default::default()
                }],
            );
            Ok(())
        }) // sync_commands end
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn power_of_two_should_round_up() {
        assert_eq!(round_to_power_of_two(1), 1);
        assert_eq!(round_to_power_of_two(2), 2);
        assert_eq!(round_to_power_of_two(3), 4);
        assert_eq!(round_to_power_of_two(6), 8);
        assert_eq!(round_to_power_of_two(9), 16);
        assert_eq!(round_to_power_of_two(20), 32);
        assert_eq!(round_to_power_of_two(50), 64);
        assert_eq!(round_to_power_of_two(93), 128);
        assert_eq!(round_to_power_of_two(200), 256);
        assert_eq!(round_to_power_of_two(500), 512);
        assert_eq!(round_to_power_of_two(10_000), 16384);
    }
}
