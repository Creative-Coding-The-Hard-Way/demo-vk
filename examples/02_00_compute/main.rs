mod compute_pipeline;

use {
    crate::compute_pipeline::Compute,
    anyhow::Result,
    ash::vk::{self},
    clap::Parser,
    demo_vk::{
        app::AppState,
        demo::{demo_main, Demo, EguiPainter, Graphics},
        graphics::{
            image_memory_barrier,
            streaming_renderer::Texture,
            vulkan::{
                raii, spirv_words, Frame, RequiredDeviceFeatures, SyncCommands,
            },
        },
        unwrap_here,
    },
    egui::Align2,
    std::sync::Arc,
    winit::{
        dpi::PhysicalSize,
        event::{ElementState, WindowEvent},
        keyboard::{KeyCode, PhysicalKey},
        window::{Fullscreen, Window},
    },
};

#[derive(Debug, Parser)]
struct Args {}

struct Example {
    gui: EguiPainter,
    kernel: Compute,
    image: Arc<Texture>,
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
            physical_device_maintenance4_features:
                vk::PhysicalDeviceMaintenance4Features {
                    maintenance4: vk::TRUE,
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
        let gui =
            unwrap_here!("Create EGUI painter", EguiPainter::new(gfx, window));

        let image = Arc::new(unwrap_here!(
            "Create compute target image",
            Texture::builder()
                .ctx(&gfx.vulkan)
                .dimensions((
                    gfx.swapchain.extent().width,
                    gfx.swapchain.extent().height
                ))
                .format(vk::Format::R16G16B16A16_SFLOAT)
                .image_usage_flags(
                    vk::ImageUsageFlags::STORAGE
                        | vk::ImageUsageFlags::SAMPLED
                        | vk::ImageUsageFlags::TRANSFER_SRC
                        | vk::ImageUsageFlags::TRANSFER_DST
                )
                .memory_property_flags(vk::MemoryPropertyFlags::DEVICE_LOCAL)
                .build()
        ));

        let init = {
            let module_spirv = unwrap_here!(
                "Include init spirv bytes",
                spirv_words(include_bytes!("./init.comp.spv"))
            );
            let module = unwrap_here!(
                "Create init kernel",
                raii::ShaderModule::new(
                    "Compute Kernel",
                    gfx.vulkan.device.clone(),
                    &vk::ShaderModuleCreateInfo {
                        code_size: module_spirv.len() * 4,
                        p_code: module_spirv.as_ptr(),
                        ..Default::default()
                    }
                )
            );
            unwrap_here!(
                "Create init kernel resources",
                Compute::new(&gfx.vulkan, &module)
            )
        };
        init.write_descriptor_set(&gfx.vulkan, &image);

        let one_time_commands = unwrap_here!(
            "Create one-time-submit command buffer",
            SyncCommands::new(gfx.vulkan.clone())
        );
        unwrap_here!(
            "Initialize compute image",
            one_time_commands.submit_and_wait(|command_buffer| {
                image_memory_barrier()
                    .ctx(&gfx.vulkan)
                    .command_buffer(command_buffer)
                    .image(image.image().raw)
                    .old_layout(vk::ImageLayout::UNDEFINED)
                    .new_layout(vk::ImageLayout::GENERAL)
                    .src_access_mask(vk::AccessFlags::empty())
                    .dst_access_mask(vk::AccessFlags::SHADER_WRITE)
                    .call();
                init.dispatch(&gfx.vulkan, command_buffer, &image);
                Ok(())
            })
        );

        let compute = {
            let module_spirv = unwrap_here!(
                "Include compute kernel spirv",
                spirv_words(include_bytes!("./image.comp.spv"))
            );
            let module = unwrap_here!(
                "Create compute kernel",
                raii::ShaderModule::new(
                    "Compute Kernel",
                    gfx.vulkan.device.clone(),
                    &vk::ShaderModuleCreateInfo {
                        code_size: module_spirv.len() * 4,
                        p_code: module_spirv.as_ptr(),
                        ..Default::default()
                    }
                )
            );
            unwrap_here!(
                "Create compute resources",
                Compute::new(&gfx.vulkan, &module)
            )
        };
        compute.write_descriptor_set(&gfx.vulkan, &image);

        Ok(Self {
            gui,
            image,
            kernel: compute,
        })
    }

    fn update(
        &mut self,
        #[allow(unused_variables)] window: &mut Window,
        #[allow(unused_variables)] gfx: &mut Graphics,
    ) -> Result<AppState> {
        let ui_result = self.gui.run(gfx, window, |ctx| {
            egui::Window::new("Frame Metrics")
                .anchor(Align2::LEFT_TOP, [0.0, 0.0])
                .default_open(false)
                .resizable(false)
                .show(ctx, |ui| ui.label(gfx.metrics.to_string()));
        });
        unwrap_here!("Build UI", ui_result);
        Ok(AppState::Continue)
    }

    fn draw(
        &mut self,
        _window: &mut Window,
        gfx: &mut Graphics,
        frame: &Frame,
    ) -> Result<AppState> {
        image_memory_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .image(self.image.image().raw)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::GENERAL)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_access_mask(vk::AccessFlags::SHADER_WRITE)
            .call();

        self.kernel
            .dispatch(&gfx.vulkan, frame.command_buffer(), &self.image);

        image_memory_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .image(self.image.image().raw)
            .old_layout(vk::ImageLayout::GENERAL)
            .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
            .call();

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
                self.image.image().raw,
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
                            x: self.image.width() as i32,
                            y: self.image.height() as i32,
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
            .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(
                vk::AccessFlags::COLOR_ATTACHMENT_READ
                    | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            )
            .call();

        unsafe {
            let color_attachments = [vk::RenderingAttachmentInfo {
                image_view: frame.swapchain_image_view(),
                image_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                resolve_mode: vk::ResolveModeFlags::NONE,
                load_op: vk::AttachmentLoadOp::LOAD,
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
            // draw
            unwrap_here!(
                "Draw GUI to current frame",
                self.gui.draw(gfx, frame)
            );
            gfx.vulkan.cmd_end_rendering(frame.command_buffer());
        }

        image_memory_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .image(frame.swapchain_image())
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .src_access_mask(
                vk::AccessFlags::COLOR_ATTACHMENT_READ
                    | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            )
            .dst_access_mask(vk::AccessFlags::empty())
            .call();

        Ok(AppState::Continue)
    }

    fn rebuild_swapchain_resources(
        &mut self,
        _window: &mut Window,
        gfx: &mut Graphics,
    ) -> Result<()> {
        unwrap_here!(
            "rebuild gui swapchain resources",
            self.gui.rebuild_swapchain_resources(gfx)
        );
        self.image = Arc::new(unwrap_here!(
            "Create compute target image",
            Texture::builder()
                .ctx(&gfx.vulkan)
                .dimensions((
                    gfx.swapchain.extent().width,
                    gfx.swapchain.extent().height
                ))
                .format(vk::Format::R16G16B16A16_SFLOAT)
                .image_usage_flags(
                    vk::ImageUsageFlags::STORAGE
                        | vk::ImageUsageFlags::SAMPLED
                        | vk::ImageUsageFlags::TRANSFER_SRC
                        | vk::ImageUsageFlags::TRANSFER_DST
                )
                .memory_property_flags(vk::MemoryPropertyFlags::DEVICE_LOCAL)
                .build()
        ));
        self.kernel.write_descriptor_set(&gfx.vulkan, &self.image);
        Ok(())
    }

    fn handle_window_event(
        &mut self,
        window: &mut Window,
        _gfx: &mut Graphics,
        event: WindowEvent,
    ) -> Result<AppState> {
        if self.gui.on_window_event(window, &event).consumed {
            return Ok(AppState::Continue);
        }

        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Released {
                    if event.physical_key == PhysicalKey::Code(KeyCode::Escape)
                    {
                        return Ok(AppState::Exit);
                    }
                    if event.physical_key == PhysicalKey::Code(KeyCode::Space) {
                        match window.fullscreen() {
                            None => {
                                window.set_fullscreen(Some(
                                    Fullscreen::Borderless(None),
                                ));
                            }
                            _ => {
                                window.set_fullscreen(None);
                            }
                        }
                    }
                }
            }
            _ => {}
        };
        Ok(AppState::Continue)
    }
}

fn main() {
    let _ = demo_main::<Example>();
}
