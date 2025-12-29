use {
    anyhow::Result,
    ash::vk::{self},
    clap::Parser,
    demo_vk::{
        app::AppState,
        demo::{demo_main, Demo, EguiPainter, Graphics},
        graphics::{
            image_memory_barrier,
            vulkan::{Frame, RequiredDeviceFeatures},
        },
        unwrap_here,
    },
    egui::{FontId, RichText},
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

struct Example {
    egui_painter: EguiPainter,
    show_fps: bool,
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
        let egui_painter = unwrap_here!(
            "Create the egui painter",
            EguiPainter::new(gfx, window)
        );
        Ok(Self {
            egui_painter,
            show_fps: false,
        })
    }

    fn update(
        &mut self,
        window: &mut Window,
        gfx: &mut Graphics,
    ) -> Result<AppState> {
        let before_egui = Instant::now();
        unwrap_here!(
            "Update EGUI UI",
            self.egui_painter.run(gfx, window, |ctx| {
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
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Central Panel");
                });
            })
        );
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
            unwrap_here!("Render EGUI ui", self.egui_painter.draw(gfx, frame));
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

    fn rebuild_swapchain_resources(
        &mut self,
        _window: &mut Window,
        gfx: &mut Graphics,
    ) -> Result<()> {
        self.egui_painter.rebuild_swapchain_resources(gfx)
    }

    fn handle_window_event(
        &mut self,
        window: &mut Window,
        _gfx: &mut Graphics,
        event: WindowEvent,
    ) -> Result<AppState> {
        if self.egui_painter.on_window_event(window, &event).consumed {
            return Ok(AppState::Continue);
        }

        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    return Ok(AppState::Exit);
                }
            }
            WindowEvent::Resized(PhysicalSize { .. }) => {
                // no-op
            }
            _ => {}
        };
        Ok(AppState::Continue)
    }
}

fn main() {
    let _ = demo_main::<Example>();
}
