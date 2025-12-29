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
                Mesh, StreamingRenderer, Texture, TextureAtlas, TextureLoader,
                TrianglesMesh, Vertex,
            },
            vulkan::{Frame, RequiredDeviceFeatures},
        },
        unwrap_here,
    },
    egui::{
        epaint::Primitive, FontId, ImageData, RichText, ScrollArea, TextEdit,
        ViewportInfo,
    },
    egui_winit::State,
    nalgebra::Matrix4,
    std::{collections::HashMap, f32, sync::Arc, time::Instant},
    winit::{
        dpi::PhysicalSize,
        event::WindowEvent,
        keyboard::{KeyCode, PhysicalKey},
        window::Window,
    },
};

#[derive(Debug, Parser)]
struct Args {}

pub fn ortho_projection(
    points_per_pixel: f32,
    width: f32,
    height: f32,
) -> Matrix4<f32> {
    let w = width / points_per_pixel;
    let h = height / points_per_pixel;
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
    projection: Matrix4<f32>,
    used_meshes: Vec<TrianglesMesh>,
    free_meshes: Vec<TrianglesMesh>,
    g2: StreamingRenderer,
    egui_textures: HashMap<egui::TextureId, i32>,
    show_fps: bool,
    egui_state: State,
    text: String,
}

impl Example {
    fn get_next_free_mesh(&mut self) -> TrianglesMesh {
        let mut mesh = self.free_meshes.pop().unwrap_or_else(|| {
            TrianglesMesh::new(1_000, self.g2.default_material().clone())
        });
        mesh.clear();
        mesh.set_transform(self.projection);
        mesh
    }
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
    fn new(
        window: &mut Window,
        gfx: &mut Graphics,
        _args: &Self::Args,
    ) -> Result<Self> {
        let texture_atlas = unwrap_here!(
            "create texture atlas",
            TextureAtlas::new(&gfx.vulkan)
        );

        let g2 = unwrap_here!(
            "Create streaming renderer",
            StreamingRenderer::new(
                &gfx.vulkan,
                gfx.swapchain.format(),
                &gfx.frames_in_flight,
                &texture_atlas,
            )
        );

        let egui_state = State::new(
            egui::Context::default(),
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        let PhysicalSize { width, height } = window.inner_size();
        let texture_loader = TextureLoader::new(gfx.vulkan.clone())?;

        Ok(Self {
            texture_loader,
            texture_atlas,
            projection: ortho_projection(
                egui_winit::pixels_per_point(egui_state.egui_ctx(), window),
                width as f32,
                height as f32,
            ),
            used_meshes: vec![],
            free_meshes: vec![],
            g2,
            egui_textures: HashMap::new(),
            show_fps: false,
            egui_state,
            text: String::new(),
        })
    }

    fn update(
        &mut self,
        window: &mut Window,
        gfx: &mut Graphics,
    ) -> Result<AppState> {
        let raw_input = {
            let mut viewport_info = ViewportInfo::default();
            egui_winit::update_viewport_info(
                &mut viewport_info,
                self.egui_state.egui_ctx(),
                window,
                false,
            );
            let viewport_id = self.egui_state.egui_input().viewport_id;
            self.egui_state
                .egui_input_mut()
                .viewports
                .insert(viewport_id, viewport_info);

            let PhysicalSize { width, height } = window.inner_size();
            self.egui_state.egui_input_mut().screen_rect = Some(egui::Rect {
                min: egui::pos2(0.0, 0.0),
                max: egui::pos2(width as f32, height as f32),
            });

            self.egui_state.take_egui_input(window)
        };
        let before_egui = Instant::now();
        let full_output = self.egui_state.egui_ctx().run(raw_input, |ctx| {
            egui::SidePanel::right("fps")
                .show_separator_line(false)
                .resizable(false)
                .frame(egui::Frame::NONE.inner_margin(10.0))
                .show(ctx, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            ui.toggle_value(
                                &mut self.show_fps,
                                RichText::new("Frame Metrics")
                                    .font(FontId::proportional(12.0)),
                            );
                            if self.show_fps {
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(format!(
                                            "{}",
                                            gfx.metrics
                                        ))
                                        .font(FontId::monospace(12.0))
                                        .color(egui::Color32::from_rgb(
                                            255, 255, 255,
                                        )),
                                    )
                                    .extend(),
                                );
                            }
                        },
                    );
                });
        });
        self.egui_state
            .handle_platform_output(window, full_output.platform_output);

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

        let pixels_per_point =
            egui_winit::pixels_per_point(self.egui_state.egui_ctx(), window);
        fn egui_rect_to_vk_rect(
            pixels_per_point: f32,
            rect: egui::Rect,
        ) -> vk::Rect2D {
            vk::Rect2D {
                offset: vk::Offset2D {
                    x: (pixels_per_point * rect.min.x) as i32,
                    y: (pixels_per_point * rect.min.y) as i32,
                },
                extent: vk::Extent2D {
                    width: (pixels_per_point * rect.width()) as u32,
                    height: (pixels_per_point * rect.height()) as u32,
                },
            }
        }

        // reset all meshes
        self.free_meshes.extend(self.used_meshes.drain(0..));

        // Fill mesh with geometry
        let clipped_primitives = self.egui_state.egui_ctx().tessellate(
            full_output.shapes,
            self.egui_state.egui_ctx().pixels_per_point(),
        );
        let mut triangles_mesh = self.get_next_free_mesh();
        let mut current_clip = vk::Rect2D::default();
        if let Some(primitive) = clipped_primitives.first() {
            current_clip =
                egui_rect_to_vk_rect(pixels_per_point, primitive.clip_rect);
            triangles_mesh.set_scissor(current_clip);
        }

        for clipped_primitive in clipped_primitives {
            let clip_rect = egui_rect_to_vk_rect(
                pixels_per_point,
                clipped_primitive.clip_rect,
            );
            if clip_rect != current_clip {
                // Allocate a new mesh and swap with the existing triangles_mesh
                // so it can be saved in the 'used_meshes' list while the
                // triangles_mesh remains the 'active' mesh for adding new
                // vertices.
                let mut next_mesh = self.get_next_free_mesh();
                (triangles_mesh, next_mesh) = (next_mesh, triangles_mesh);
                self.used_meshes.push(next_mesh);
                triangles_mesh.set_scissor(clip_rect);
            }

            match clipped_primitive.primitive {
                Primitive::Mesh(mesh) => {
                    let texture_id = *self
                        .egui_textures
                        .get(&mesh.texture_id)
                        .unwrap_or(&-1);
                    triangles_mesh.indexed_triangles(
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
        self.used_meshes.push(triangles_mesh);
        gfx.metrics.ms_since("EGUI", before_egui);

        Ok(AppState::Continue)
    }

    /// Draw a frame
    fn draw(
        &mut self,
        _window: &mut Window,
        gfx: &mut Graphics,
        frame: &Frame,
    ) -> Result<AppState> {
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
            self.g2.prepare_meshes(
                &gfx.vulkan,
                frame,
                &self
                    .used_meshes
                    .iter()
                    .map(|mesh| mesh as &dyn Mesh)
                    .collect::<Vec<_>>(),
            )?;
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

        Ok(AppState::Continue)
    }

    fn handle_window_event(
        &mut self,
        window: &mut Window,
        _gfx: &mut Graphics,
        event: WindowEvent,
    ) -> Result<AppState> {
        if self.egui_state.on_window_event(window, &event).consumed {
            return Ok(AppState::Continue);
        }

        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    return Ok(AppState::Exit);
                }
            }
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                self.projection = ortho_projection(
                    egui_winit::pixels_per_point(
                        self.egui_state.egui_ctx(),
                        window,
                    ),
                    width as f32,
                    height as f32,
                );
            }
            _ => {}
        };
        Ok(AppState::Continue)
    }
}

fn main() {
    let _ = demo_main::<Example>();
}
