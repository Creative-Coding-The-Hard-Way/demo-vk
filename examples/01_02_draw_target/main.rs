use {
    anyhow::{Context, Result},
    ash::vk::{self},
    clap::Parser,
    demo_vk::{
        app::AppState,
        demo::{demo_main, Demo, Graphics},
        graphics::{
            image_memory_barrier,
            streaming_renderer::{
                StreamingRenderer, Texture, TextureAtlas, TrianglesMesh,
            },
            vulkan::{Frame, RequiredDeviceFeatures},
        },
        unwrap_here,
    },
    nalgebra::Matrix4,
    std::time::Instant,
    winit::{
        dpi::PhysicalSize,
        event::WindowEvent,
        keyboard::{KeyCode, PhysicalKey},
        window::Window,
    },
};

#[derive(Debug, Parser)]
struct Args {}

pub fn ortho_projection(aspect: f32, height: f32) -> Matrix4<f32> {
    let w = height * aspect;
    let h = height;
    #[rustfmt::skip]
    let projection = Matrix4::new(
        2.0 / w,  0.0,     0.0, 0.0,
        0.0,     -2.0 / h, 0.0, 0.0,
        0.0,      0.0,     1.0, 0.0,
        0.0,      0.0,     0.0, 1.0,
    );
    projection
}

struct Example {
    texture_atlas: TextureAtlas,
    mesh: TrianglesMesh,
    g2: StreamingRenderer,
    draw_target: Texture,
    start: Instant,
}

impl Demo for Example {
    type Args = Args;
    const FRAMES_IN_FLIGHT_COUNT: usize = 2;

    fn required_device_features() -> RequiredDeviceFeatures {
        RequiredDeviceFeatures {
            physical_device_dynamic_rendering_features:
                vk::PhysicalDeviceDynamicRenderingFeatures {
                    dynamic_rendering: vk::TRUE,
                    ..Default::default()
                },
            physical_device_vulkan12_features:
                vk::PhysicalDeviceVulkan12Features {
                    // required for texture atlas behavior
                    runtime_descriptor_array: vk::TRUE,
                    descriptor_indexing: vk::TRUE,
                    descriptor_binding_variable_descriptor_count: vk::TRUE,
                    descriptor_binding_update_unused_while_pending: vk::TRUE,
                    descriptor_binding_partially_bound: vk::TRUE,
                    descriptor_binding_sampled_image_update_after_bind:
                        vk::TRUE,

                    // required for mesh buffers (vertex and transforms)
                    buffer_device_address: vk::TRUE,
                    ..Default::default()
                },
            ..Default::default()
        }
    }

    fn new(
        _window: &mut Window,
        gfx: &mut Graphics,
        _args: &Self::Args,
    ) -> Result<Self> {
        let texture_atlas = unwrap_here!(
            "Create texture atlas.",
            TextureAtlas::new(&gfx.vulkan)
        );

        let g2 = unwrap_here!(
            "Create streaming renderer",
            StreamingRenderer::new(
                &gfx.vulkan,
                vk::Format::R16G16B16A16_SFLOAT,
                &gfx.frames_in_flight,
                &texture_atlas,
            )
        );

        let mesh = {
            let mut mesh =
                TrianglesMesh::new(10, g2.default_material().clone());
            mesh.set_transform(ortho_projection(
                gfx.swapchain.extent().width as f32
                    / gfx.swapchain.extent().height as f32,
                1.0,
            ));
            mesh.set_scissor(vk::Rect2D {
                extent: gfx.swapchain.extent(),
                ..Default::default()
            });
            mesh
        };

        let draw_target = unwrap_here!(
            "Create high precision offscreen render target",
            Texture::builder()
                .ctx(&gfx.vulkan)
                .image_usage_flags(
                    vk::ImageUsageFlags::COLOR_ATTACHMENT
                        | vk::ImageUsageFlags::TRANSFER_SRC,
                )
                .memory_property_flags(vk::MemoryPropertyFlags::DEVICE_LOCAL)
                .dimensions((
                    gfx.swapchain.extent().width,
                    gfx.swapchain.extent().height,
                ))
                .format(vk::Format::R16G16B16A16_SFLOAT)
                .build()
        );

        Ok(Self {
            texture_atlas,
            mesh,
            g2,
            draw_target,
            start: Instant::now(),
        })
    }

    fn update(
        &mut self,
        _window: &mut Window,
        _gfx: &mut Graphics,
    ) -> Result<AppState> {
        self.mesh.clear();

        let z = 0.0;
        self.mesh.quad(
            [0.0, 0.0, 0.0, 1.0],
            -1,
            nalgebra::vector![-0.5, 0.5, z],
            nalgebra::vector![0.5, 0.5, z],
            nalgebra::vector![0.5, -0.5, z],
            nalgebra::vector![-0.5, -0.5, z],
        );

        let dt = Instant::now().duration_since(self.start).as_secs_f32();
        let point = nalgebra::vector![0.45 * dt.cos(), 0.45 * dt.sin(), z];
        self.mesh.quad(
            [
                dt.cos().abs(),
                (dt * 2.0).sin().abs(),
                (dt / 3.0).tan().abs(),
                1.0,
            ],
            -1,
            point + nalgebra::vector![-0.05, 0.05, z],
            point + nalgebra::vector![0.05, 0.05, z],
            point + nalgebra::vector![0.05, -0.05, z],
            point + nalgebra::vector![-0.05, -0.05, z],
        );

        Ok(AppState::Continue)
    }

    /// Draw a frame
    fn draw(
        &mut self,
        _window: &mut Window,
        gfx: &mut Graphics,
        frame: &Frame,
    ) -> Result<AppState> {
        // render to the draw target

        image_memory_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .image(self.draw_target.image().raw)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .call();

        unsafe {
            let color_attachments = [vk::RenderingAttachmentInfo {
                image_view: self.draw_target.view().raw,
                image_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                resolve_mode: vk::ResolveModeFlags::NONE,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.7, 0.7, 0.7, 1.0],
                    },
                },
                ..Default::default()
            }];
            gfx.vulkan.cmd_begin_rendering(
                frame.command_buffer(),
                &vk::RenderingInfo {
                    render_area: vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: self.draw_target.extent(),
                    },
                    layer_count: 1,
                    color_attachment_count: 1,
                    p_color_attachments: color_attachments.as_ptr(),
                    ..Default::default()
                },
            );
            gfx.vulkan.cmd_set_viewport(
                frame.command_buffer(),
                0,
                &[vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: self.draw_target.width() as f32,
                    height: self.draw_target.height() as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                }],
            );
            self.g2
                .bind_texture_atlas(&gfx.vulkan, frame, &self.texture_atlas);
            self.g2.prepare_meshes(&gfx.vulkan, frame, &[&self.mesh])?;
            self.g2.write_draw_commands(&gfx.vulkan, frame)?;
            gfx.vulkan.cmd_end_rendering(frame.command_buffer());
        }

        image_memory_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .image(self.draw_target.image().raw)
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
            .call();

        // blit the draw target onto the swapchain image
        image_memory_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .image(frame.swapchain_image())
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .call();

        unsafe {
            gfx.vulkan.cmd_blit_image(
                frame.command_buffer(),
                self.draw_target.image().raw,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                frame.swapchain_image(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[vk::ImageBlit {
                    src_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        mip_level: 0,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    src_offsets: [
                        vk::Offset3D::default(),
                        vk::Offset3D {
                            x: self.draw_target.width() as i32,
                            y: self.draw_target.height() as i32,
                            z: 1,
                        },
                    ],
                    dst_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        mip_level: 0,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    dst_offsets: [
                        vk::Offset3D::default(),
                        vk::Offset3D {
                            x: gfx.swapchain.extent().width as i32,
                            y: gfx.swapchain.extent().height as i32,
                            z: 1,
                        },
                    ],
                }],
                vk::Filter::LINEAR,
            );
        }

        image_memory_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .image(frame.swapchain_image())
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::empty())
            .call();

        Ok(AppState::Continue)
    }

    fn handle_window_event(
        &mut self,
        _window: &mut Window,
        _gfx: &mut Graphics,
        event: WindowEvent,
    ) -> Result<AppState> {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    return Ok(AppState::Exit);
                }
            }
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                self.mesh.set_transform(ortho_projection(
                    width as f32 / height as f32,
                    1.0,
                ));
                self.mesh.set_scissor(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: vk::Extent2D {
                        width: width as u32,
                        height: height as u32,
                    },
                });
            }
            _ => {}
        };
        Ok(AppState::Continue)
    }

    fn rebuild_swapchain_resources(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Graphics,
    ) -> Result<()> {
        // safe because no frame are in flight
        self.draw_target = unwrap_here!(
            "Recreate the high precision offscreen render target",
            Texture::builder()
                .ctx(&gfx.vulkan)
                .image_usage_flags(
                    vk::ImageUsageFlags::COLOR_ATTACHMENT
                        | vk::ImageUsageFlags::TRANSFER_SRC,
                )
                .memory_property_flags(vk::MemoryPropertyFlags::DEVICE_LOCAL)
                .dimensions((
                    gfx.swapchain.extent().width,
                    gfx.swapchain.extent().height,
                ))
                .format(vk::Format::R16G16B16A16_SFLOAT)
                .build()
        );
        Ok(())
    }
}

fn main() {
    let _ = demo_main::<Example>();
}
