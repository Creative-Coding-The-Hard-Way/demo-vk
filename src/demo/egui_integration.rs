use {
    crate::{
        demo::Graphics,
        graphics::{
            streaming_renderer::{
                Mesh, StreamingRenderer, Texture, TextureAtlas, TextureLoader,
                TrianglesMesh, Vertex,
            },
            vulkan::Frame,
        },
        unwrap_here,
    },
    anyhow::Result,
    ash::vk,
    egui::{epaint::Primitive, ImageData, ViewportInfo},
    egui_winit::EventResponse,
    nalgebra::Matrix4,
    std::{collections::HashMap, sync::Arc},
    winit::{dpi::PhysicalSize, event::WindowEvent, window::Window},
};

/// The EGUI painter is a complete EGUI integration for use within demos.
///
/// It maintains internal state for rendering directly to the swapchain image,
/// and processing Winit events.
pub struct EguiPainter {
    state: egui_winit::State,
    projection: Matrix4<f32>,
    used_meshes: Vec<TrianglesMesh>,
    free_meshes: Vec<TrianglesMesh>,
    egui_textures: HashMap<egui::TextureId, i32>,
    texture_loader: TextureLoader,
    atlas: TextureAtlas,
    renderer: StreamingRenderer,
}

/// Builds a projection matrix for the screen based on the current
/// points_per_pixel and screen dimensions. This ensures that EGUI UI items
/// correctly adhere to display scaling requirements for high-dpi displays.
fn ortho_projection(
    pixels_per_point: f32,
    width: f32,
    height: f32,
) -> Matrix4<f32> {
    let w = width / pixels_per_point;
    let h = height / pixels_per_point;
    #[rustfmt::skip]
    let projection = Matrix4::new(
        2.0 / w,  0.0,     0.0, -1.0,
        0.0,     2.0 / h, 0.0, -1.0,
        0.0,      0.0,     1.0, 0.0,
        0.0,      0.0,     0.0, 1.0,
    );
    projection
}

/// Converts an EGUI clipping rect into a Vulkan vk::Rect2D, accounting for
/// the pixels_per_point setting used by EGUI to scale UI elements.
fn egui_rect_to_vk_rect(pixels_per_point: f32, rect: egui::Rect) -> vk::Rect2D {
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

impl EguiPainter {
    pub fn new(graphics: &Graphics, window: &Window) -> Result<Self> {
        let state = egui_winit::State::new(
            egui::Context::default(),
            egui::ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );
        let pixels_per_point =
            egui_winit::pixels_per_point(state.egui_ctx(), window);
        let PhysicalSize { width, height } = window.inner_size();
        let atlas = unwrap_here!(
            "Create EGUI painter texture atlas",
            TextureAtlas::new(&graphics.vulkan)
        );
        let texture_loader = unwrap_here!(
            "Create EGUI texture loader",
            TextureLoader::new(graphics.vulkan.clone())
        );
        let renderer = unwrap_here!(
            "Create EGUI painter streaming renderer instance",
            StreamingRenderer::new(
                &graphics.vulkan,
                graphics.swapchain.format(),
                &graphics.frames_in_flight,
                &atlas
            )
        );
        Ok(Self {
            state,
            projection: ortho_projection(
                pixels_per_point,
                width as f32,
                height as f32,
            ),
            used_meshes: vec![],
            free_meshes: vec![],
            egui_textures: HashMap::new(),
            texture_loader,
            atlas,
            renderer,
        })
    }

    /// Processes a window event so the UI will respond to clicks and
    /// keystrokes.
    pub fn on_window_event(
        &mut self,
        window: &Window,
        event: &WindowEvent,
    ) -> EventResponse {
        if let &WindowEvent::Resized(PhysicalSize { width, height }) = event {
            self.projection = ortho_projection(
                egui_winit::pixels_per_point(self.state.egui_ctx(), window),
                width as f32,
                height as f32,
            );
        }
        self.state.on_window_event(window, event)
    }

    /// Checks for compatibility with the swapchain and rebuilds resources if
    /// needed.
    pub fn rebuild_swapchain_resources(
        &mut self,
        gfx: &Graphics,
    ) -> Result<()> {
        if self.renderer.image_format() == gfx.swapchain.format() {
            return Ok(()); // formats are compatible, nothing to be donwe
        }

        unwrap_here!(
            "Wait for frames to finish before rebuilding EGUI painter resources",
            gfx.frames_in_flight.wait_for_all_frames_to_complete()
        );

        // free all existing meshes and associated materials
        self.free_meshes.clear();
        self.used_meshes.clear();

        self.renderer = unwrap_here!(
            "Rebuild streaming renderer for the EGUI painter",
            StreamingRenderer::new(
                &gfx.vulkan,
                gfx.swapchain.format(),
                &gfx.frames_in_flight,
                &self.atlas
            )
        );

        // resources must be rebuilt
        Ok(())
    }

    /// Draws the EGUI UI to the currently bound color attachment.
    ///
    /// # Safety
    ///
    /// - This function assumes that a render pass is already started (dynamic
    ///   or not)
    /// - This function assumes that the color attachment 0 is either a
    ///   swapchain image or an image with the same dimensions and format.
    /// - This function assumes that the viewport has already been set.
    pub unsafe fn draw(&mut self, gfx: &Graphics, frame: &Frame) -> Result<()> {
        self.renderer
            .bind_texture_atlas(&gfx.vulkan, frame, &self.atlas);
        unwrap_here!(
            "Prepare EGUI clipped primitive meshes",
            self.renderer.prepare_meshes(
                &gfx.vulkan,
                frame,
                &self
                    .used_meshes
                    .iter()
                    .map(|mesh| mesh as &dyn Mesh)
                    .collect::<Vec<_>>(),
            )
        );
        unwrap_here!(
            "Write EGUI draw commands to frame command buffer",
            self.renderer.write_draw_commands(&gfx.vulkan, frame)
        );
        Ok(())
    }

    pub fn run(
        &mut self,
        gfx: &Graphics,
        window: &Window,
        run_ui: impl FnMut(&egui::Context),
    ) -> Result<()> {
        let raw_input = {
            let mut viewport_info = ViewportInfo::default();
            egui_winit::update_viewport_info(
                &mut viewport_info,
                self.state.egui_ctx(),
                window,
                false,
            );
            let viewport_id = self.state.egui_input().viewport_id;
            self.state
                .egui_input_mut()
                .viewports
                .insert(viewport_id, viewport_info);

            let PhysicalSize { width, height } = window.inner_size();
            self.state.egui_input_mut().screen_rect = Some(egui::Rect {
                min: egui::pos2(0.0, 0.0),
                max: egui::pos2(width as f32, height as f32),
            });

            self.state.take_egui_input(window)
        };
        let full_output = self.state.egui_ctx().run(raw_input, run_ui);
        self.state
            .handle_platform_output(window, full_output.platform_output);

        // create new textures if needed
        for (egui_texture_id, delta) in &full_output.textures_delta.set {
            let needs_replaced = if let Some(existing_texture_id) =
                self.egui_textures.get(egui_texture_id)
            {
                // a texture exists, check to see if it needs to be reallocated
                // based on the delta size
                let existing_texture =
                    self.atlas.get_texture(*existing_texture_id);

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

                let texture = unwrap_here!(
                    "Create EGUI texture",
                    Texture::builder()
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
                );

                self.egui_textures.insert(
                    *egui_texture_id,
                    self.atlas.add_texture(&gfx.vulkan, Arc::new(texture)),
                );
            }
        }

        // The previous block created any new textures that might be needed, so
        // here we just need to write texture data.
        for (egui_texture_id, delta) in full_output.textures_delta.set {
            let texture_id = if let Some(texture_id) =
                self.egui_textures.get(&egui_texture_id)
            {
                *texture_id
            } else {
                continue;
            };
            let texture = self.atlas.get_texture(texture_id);
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
            unwrap_here!(
                "Update image data in EGUI texture",
                self.texture_loader.tex_sub_image(
                    &gfx.vulkan,
                    texture,
                    &pixels,
                    offset,
                    size,
                )
            );
        }

        let pixels_per_point = full_output.pixels_per_point;

        // reset all meshes
        self.free_meshes.extend(self.used_meshes.drain(0..));

        // Fill mesh with geometry
        let clipped_primitives = self.state.egui_ctx().tessellate(
            full_output.shapes,
            self.state.egui_ctx().pixels_per_point(),
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
                        mesh.vertices.iter().map(|egui_vertex| {
                            Vertex::new(
                                [egui_vertex.pos.x, egui_vertex.pos.y, 0.0],
                                [egui_vertex.uv.x, egui_vertex.uv.y],
                                egui_vertex.color.to_normalized_gamma_f32(),
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

        Ok(())
    }

    fn get_next_free_mesh(&mut self) -> TrianglesMesh {
        let mut mesh = self.free_meshes.pop().unwrap_or_else(|| {
            TrianglesMesh::new(1_000, self.renderer.default_material().clone())
        });
        mesh.clear();
        mesh.set_transform(self.projection);
        mesh
    }
}
