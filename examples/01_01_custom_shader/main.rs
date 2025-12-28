use {
    anyhow::Result,
    ash::vk::{self},
    clap::Parser,
    demo_vk::{
        app::AppState,
        demo::{demo_main, Demo, Graphics},
        graphics::{
            image_memory_barrier,
            streaming_renderer::{
                StreamingRenderer, TextureAtlas, TrianglesMesh,
            },
            vulkan::{raii, spirv_words, Frame, RequiredDeviceFeatures},
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

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct FrameData {
    delta_time: f32,
    current_time: f32,
}

struct Example {
    texture_atlas: TextureAtlas,
    mesh: TrianglesMesh,
    g2: StreamingRenderer<FrameData>,
    app_start: Instant,
    last_frame: Instant,
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
            "Create texture atlas",
            TextureAtlas::new(&gfx.vulkan)
        );

        let g2 = unwrap_here!(
            "Create streaming renderer with custom frame data",
            StreamingRenderer::<FrameData>::new(
                &gfx.vulkan,
                gfx.swapchain.format(),
                &gfx.frames_in_flight,
                &texture_atlas,
            )
        );

        let mesh = {
            let fragment_shader_spirv =
                spirv_words(include_bytes!("./triangle.frag.spv"))?;
            let fragment_module = raii::ShaderModule::new(
                "Custom Fragment Module",
                gfx.vulkan.device.clone(),
                &vk::ShaderModuleCreateInfo {
                    code_size: fragment_shader_spirv.len() * 4,
                    p_code: fragment_shader_spirv.as_ptr(),
                    ..Default::default()
                },
            )?;
            let mut mesh = TrianglesMesh::new(
                10,
                g2.new_material(&gfx.vulkan, None, Some(&fragment_module))?,
            );
            mesh.set_transform(ortho_projection(1.0, 1.0));
            mesh.set_scissor(vk::Rect2D {
                extent: gfx.swapchain.extent(),
                ..Default::default()
            });
            mesh
        };

        Ok(Self {
            texture_atlas,
            mesh,
            g2,
            app_start: Instant::now(),
            last_frame: Instant::now(),
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
            [1.0, 1.0, 1.0, 1.0],
            -1,
            nalgebra::vector![-0.5, 0.5, z],
            nalgebra::vector![0.5, 0.5, z],
            nalgebra::vector![0.5, -0.5, z],
            nalgebra::vector![-0.5, -0.5, z],
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
        self.g2.set_frame_constants(
            frame,
            FrameData {
                delta_time: Instant::now()
                    .duration_since(self.last_frame)
                    .as_secs_f32(),
                current_time: Instant::now()
                    .duration_since(self.app_start)
                    .as_secs_f32(),
            },
        )?;
        self.last_frame = Instant::now();

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
                self.mesh.set_transform(ortho_projection(1.0, 1.0));
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
}

fn main() {
    let _ = demo_main::<Example>();
}
