use {
    anyhow::{Context, Result},
    ash::vk::{self},
    clap::Parser,
    demo_vk::{
        demo::{demo_main, Demo, Graphics},
        graphics::{
            image_memory_barrier,
            streaming_renderer::{
                StreamingRenderer, Texture, TextureAtlas, TextureLoader,
                TrianglesMesh, Vertex,
            },
            vulkan::{Frame, RequiredDeviceFeatures},
        },
    },
    egui::{epaint::Primitive, FontId, ImageData, RichText},
    glfw::{Action, Key, Modifiers, Window, WindowEvent},
    nalgebra::Matrix4,
    std::{collections::HashMap, f32, sync::Arc},
};

#[derive(Debug, Parser)]
struct Args {}

type Gfx = Graphics<Args>;

pub fn ortho_projection(width: f32, height: f32) -> Matrix4<f32> {
    let w = width;
    let h = height;
    #[rustfmt::skip]
    let projection = Matrix4::new(
        2.0 / w,  0.0,     0.0, -1.0,
        0.0,     2.0 / h, 0.0, -1.0,
        0.0,      0.0,     1.0, 0.0,
        0.0,      0.0,     0.0, 1.0,
    );
    projection
}

struct Example {
    texture_atlas: TextureAtlas,
    texture_loader: TextureLoader,
    mesh: TrianglesMesh,
    g2: StreamingRenderer,
    input: egui::RawInput,
    egui_ctx: egui::Context,
    egui_textures: HashMap<egui::TextureId, i32>,
    slider_value: f32,
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

    /// Initialize the demo
    fn new(window: &mut Window, gfx: &mut Gfx) -> Result<Self> {
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_char_polling(true);
        window.set_size(1920, 1080);

        let texture_atlas = TextureAtlas::new(&gfx.vulkan)
            .context("Unable to create texture atlas")?;

        let g2 = StreamingRenderer::new(
            &gfx.vulkan,
            gfx.swapchain.format(),
            &gfx.frames_in_flight,
            &texture_atlas,
        )
        .context("Unable to create g2 subsystem")?;

        let mesh = {
            let mut mesh =
                TrianglesMesh::new(10, g2.default_material().clone());
            let (w, h) = window.get_size();
            mesh.set_transform(ortho_projection(w as f32, h as f32));
            mesh.set_scissor(vk::Rect2D {
                extent: gfx.swapchain.extent(),
                ..Default::default()
            });
            mesh
        };

        let texture_loader = TextureLoader::new(gfx.vulkan.clone())?;

        let egui_ctx = egui::Context::default();
        egui_ctx.set_pixels_per_point(window.get_content_scale().0);
        log::info!("pixels per point: {}", egui_ctx.pixels_per_point());

        Ok(Self {
            texture_loader,
            texture_atlas,
            mesh,
            g2,
            input: egui::RawInput {
                screen_rect: Some(egui::Rect {
                    min: egui::pos2(0.0, 0.0),
                    max: egui::pos2(
                        gfx.swapchain.extent().width as f32,
                        gfx.swapchain.extent().height as f32,
                    ),
                }),
                ..egui::RawInput::default()
            },
            egui_ctx,
            egui_textures: HashMap::new(),
            slider_value: 0.0,
        })
    }

    fn update(&mut self, _window: &mut Window, gfx: &mut Gfx) -> Result<()> {
        let full_output = self.egui_ctx.run(self.input.take(), |ctx| {
            egui::SidePanel::left("settings").show(ctx, |ui| {
                ui.label(
                    RichText::new("Hello World")
                        .font(FontId::proportional(24.0))
                        .color(egui::Color32::from_rgb(0, 0, 0)),
                );
                ui.add(
                    egui::widgets::Slider::new(
                        &mut self.slider_value,
                        0.0..=10.0,
                    )
                    .step_by(2.5)
                    .text(
                        RichText::new("Whatt??")
                            .font(FontId::proportional(16.0))
                            .color(egui::Color32::from_rgb(0, 0, 0)),
                    ),
                );
            });
        });

        // create new textures if needed
        for (egui_texture_id, delta) in &full_output.textures_delta.set {
            let needs_replaced = if let Some(existing_texture_id) =
                self.egui_textures.get(egui_texture_id)
            {
                // a texture exists, check to see if it needs to be reallocated
                // based on the delta size
                let existing_texture =
                    self.texture_atlas.get_texture(*existing_texture_id);

                (existing_texture.width() as usize) < delta.image.width()
                    || (existing_texture.height() as usize)
                        < delta.image.height()
            } else {
                true // texture doesn't exist, so it always "needs replaced"
            };

            // create a new texture and update the texture map
            if needs_replaced {
                // no existing texture, so create a new one
                log::info!("Creating egui texture: {:?}", egui_texture_id);

                let texture = Texture::builder()
                    .ctx(&gfx.vulkan)
                    .dimensions((
                        delta.image.width() as u32,
                        delta.image.height() as u32,
                    ))
                    .format(vk::Format::R8G8B8A8_UNORM)
                    .memory_property_flags(
                        vk::MemoryPropertyFlags::DEVICE_LOCAL,
                    )
                    .image_usage_flags(
                        vk::ImageUsageFlags::TRANSFER_DST
                            | vk::ImageUsageFlags::SAMPLED,
                    )
                    .build()
                    .with_context(|| {
                        format!(
                            "Unable to create texture for egui: {:?}",
                            egui_texture_id,
                        )
                    })?;

                let id = self
                    .texture_atlas
                    .add_texture(&gfx.vulkan, Arc::new(texture));

                self.egui_textures.insert(*egui_texture_id, id);
            }
        }

        // Update created any new textures that might be needed, so here we just
        // need to write texture data.
        for (egui_texture_id, delta) in full_output.textures_delta.set {
            let texture_id = if let Some(texture_id) =
                self.egui_textures.get(&egui_texture_id)
            {
                *texture_id
            } else {
                continue;
            };
            let texture = self.texture_atlas.get_texture(texture_id);
            let ImageData::Color(color) = delta.image;
            let pixels = color
                .pixels
                .iter()
                .flat_map(|color| color.to_srgba_unmultiplied())
                .collect::<Vec<u8>>();
            let offset = if let Some([w, h]) = delta.pos {
                [w as u32, h as u32]
            } else {
                [0, 0]
            };
            let size = {
                let [w, h] = color.size;
                [w as u32, h as u32]
            };
            self.texture_loader.tex_sub_image(
                &gfx.vulkan,
                texture,
                &pixels,
                offset,
                size,
            )?;
        }

        // Fill mesh with geometry
        self.mesh.clear();
        let clipped_primitives = self
            .egui_ctx
            .tessellate(full_output.shapes, self.egui_ctx.pixels_per_point());
        for clipped_primitive in clipped_primitives {
            match clipped_primitive.primitive {
                Primitive::Mesh(mesh) => {
                    let texture_id = *self
                        .egui_textures
                        .get(&mesh.texture_id)
                        .unwrap_or(&-1);
                    self.mesh.indexed_triangles(
                        mesh.vertices.iter().map(|vertex| {
                            Vertex::new(
                                [vertex.pos.x, vertex.pos.y, 0.0],
                                [vertex.uv.x, vertex.uv.y],
                                vertex.color.to_normalized_gamma_f32(),
                                texture_id,
                            )
                        }),
                        mesh.indices.iter().copied(),
                    );
                }
                Primitive::Callback(_) => {
                    log::warn!(
                        "Callbacks unsupported by this backend (currently)"
                    );
                }
            }
        }

        Ok(())
    }

    /// Draw a frame
    fn draw(
        &mut self,
        _window: &mut Window,
        gfx: &mut Gfx,
        frame: &Frame,
    ) -> Result<()> {
        image_memory_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .image(frame.swapchain_image())
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .call();

        unsafe {
            let color_attachments = [vk::RenderingAttachmentInfo {
                image_view: frame.swapchain_image_view(),
                image_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                resolve_mode: vk::ResolveModeFlags::NONE,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                },
                ..Default::default()
            }];
            gfx.vulkan.cmd_begin_rendering(
                frame.command_buffer(),
                &vk::RenderingInfo {
                    render_area: vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: gfx.swapchain.extent(),
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
                    width: gfx.swapchain.extent().width as f32,
                    height: gfx.swapchain.extent().height as f32,
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
            .image(frame.swapchain_image())
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .dst_access_mask(vk::AccessFlags::empty())
            .call();

        Ok(())
    }

    fn handle_event(
        &mut self,
        window: &mut Window,
        _gfx: &mut Gfx,
        event: WindowEvent,
    ) -> Result<()> {
        let egui_event = match event {
            WindowEvent::CursorPos(x, y) => {
                Some(egui::Event::PointerMoved(egui::pos2(x as f32, y as f32)))
            }
            WindowEvent::MouseButton(button, action, modifiers) => {
                let (x, y) = window.get_cursor_pos();
                Some(egui::Event::PointerButton {
                    pos: egui::pos2(x as f32, y as f32),
                    button: match button {
                        glfw::MouseButtonLeft => egui::PointerButton::Primary,
                        glfw::MouseButtonRight => {
                            egui::PointerButton::Secondary
                        }
                        glfw::MouseButtonMiddle => egui::PointerButton::Middle,
                        _ => egui::PointerButton::Primary,
                    },
                    pressed: match action {
                        Action::Press => true,
                        Action::Release => false,
                        Action::Repeat => true,
                    },
                    modifiers: egui::Modifiers {
                        alt: modifiers.contains(Modifiers::Alt),
                        ctrl: modifiers.contains(Modifiers::Control),
                        shift: modifiers.contains(Modifiers::Shift),
                        mac_cmd: modifiers.contains(Modifiers::Super),
                        command: modifiers.contains(Modifiers::Super),
                    },
                })
            }
            WindowEvent::Key(key, _, action, modifiers) => {
                log::info!("original key {:?}", key);
                key.get_name()
                    .and_then(|key| egui::Key::from_name(&key))
                    .map(|key| {
                        log::info!("Key event {:?}", key);
                        egui::Event::Key {
                            key,
                            physical_key: None,
                            pressed: match action {
                                Action::Press => true,
                                Action::Repeat => true,
                                Action::Release => false,
                            },
                            repeat: action == Action::Repeat,
                            modifiers: egui::Modifiers {
                                alt: modifiers.contains(Modifiers::Alt),
                                ctrl: modifiers.contains(Modifiers::Control),
                                shift: modifiers.contains(Modifiers::Shift),
                                mac_cmd: modifiers.contains(Modifiers::Super),
                                command: modifiers.contains(Modifiers::Super),
                            },
                        }
                    })
            }
            WindowEvent::Char(char) => {
                Some(egui::Event::Text(char.to_string()))
            }
            _ => None,
        };
        if let Some(egui_event) = egui_event {
            self.input.events.push(egui_event);
        }

        match event {
            WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                window.set_should_close(true);
            }
            WindowEvent::FramebufferSize(
                framebuffer_width,
                framebuffer_height,
            ) => {
                let (screen_width, screen_height) = window.get_size();
                log::info!(
                    "FramebufferSize: {}x{}, ScreenSize: {}x{}",
                    framebuffer_width,
                    framebuffer_height,
                    screen_width,
                    screen_height,
                );
                self.mesh.set_transform(ortho_projection(
                    screen_width as f32,
                    screen_height as f32,
                ));
                self.mesh.set_scissor(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: vk::Extent2D {
                        width: framebuffer_width as u32,
                        height: framebuffer_height as u32,
                    },
                });
                self.input.screen_rect = Some(egui::Rect {
                    min: egui::pos2(0.0, 0.0),
                    max: egui::pos2(screen_width as f32, screen_height as f32),
                });
            }
            _ => {}
        };
        Ok(())
    }
}

fn main() {
    let _ = demo_main::<Example>();
}
