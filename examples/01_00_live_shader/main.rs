use {
    anyhow::Result,
    clap::Parser,
    demo_vk::{
        app::FullscreenToggle,
        demo::{demo_main, Demo, Graphics},
        graphics::{
            BindlessTextureAtlas, Recompiler, Sprite, SpriteLayer,
            StreamingSprites, SwapchainColorPass,
        },
    },
    glfw::{Action, Key, WindowEvent},
    nalgebra::Matrix4,
    std::{path::PathBuf, time::Instant},
};

#[derive(Debug, Copy, Clone, Default)]
#[repr(C, packed)]
struct FrameData {
    screen_size: [f32; 2],
    t: f32,
    pad: u32,
}

#[derive(Parser, Debug)]
struct Args {
    /// The path to the fragment shader's source.
    #[arg(short, long)]
    fragment_shader: PathBuf,
}

/// A sprites demo.
struct Sprites {
    start_time: Instant,

    atlas: BindlessTextureAtlas,
    layer: SpriteLayer<FrameData>,
    fullscreen_quad: StreamingSprites,
    color_pass: SwapchainColorPass,
    fragment_shader_compiler: Recompiler,

    fullscreen_toggle: FullscreenToggle,
}

impl Demo for Sprites {
    type Args = Args;

    fn new(
        window: &mut glfw::Window,
        gfx: &mut Graphics<Args>,
    ) -> Result<Self> {
        window.set_all_polling(true);

        let ctx = &gfx.vulkan;

        let fragment_shader_compiler =
            Recompiler::new(ctx.clone(), &gfx.args.fragment_shader, &[])?;

        let color_pass = SwapchainColorPass::new(ctx.clone(), &gfx.swapchain)?;
        let atlas =
            BindlessTextureAtlas::new(ctx.clone(), 1, &gfx.frames_in_flight)?;

        let layer = SpriteLayer::builder()
            .ctx(ctx.clone())
            .frames_in_flight(&gfx.frames_in_flight)
            .texture_atlas_layout(atlas.descriptor_set_layout())
            .render_pass(color_pass.renderpass())
            .projection(Matrix4::identity())
            .fragment_shader(fragment_shader_compiler.shader())
            .build()?;

        let fullscreen_quad = StreamingSprites::builder()
            .ctx(ctx.clone())
            .frames_in_flight(&gfx.frames_in_flight)
            .viewport(gfx.swapchain.viewport())
            .scissor(gfx.swapchain.scissor())
            .build()?;

        Ok(Self {
            start_time: Instant::now(),

            layer,
            fullscreen_quad,
            atlas,
            color_pass,
            fragment_shader_compiler,

            fullscreen_toggle: FullscreenToggle::new(window),
        })
    }

    fn handle_event(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
        #[allow(unused_variables)] event: glfw::WindowEvent,
    ) -> Result<()> {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                window.set_should_close(true);
            }
            WindowEvent::Key(Key::Space, _, Action::Release, _) => {
                self.fullscreen_toggle.toggle_fullscreen(window)?;
            }
            _ => (),
        }
        Ok(())
    }

    fn update(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
    ) -> Result<()> {
        if self.fragment_shader_compiler.check_for_update()? {
            gfx.frames_in_flight.wait_for_all_frames_to_complete()?;
            unsafe {
                // Safe because all frames are complete
                self.layer.rebuild_pipeline(
                    self.color_pass.renderpass(),
                    Some(self.fragment_shader_compiler.shader()),
                )?;
            }
        }

        let (w, h) = window.get_framebuffer_size();
        let t = Instant::now().duration_since(self.start_time).as_secs_f32();
        self.layer.set_user_data(FrameData {
            t,
            screen_size: [w as f32, h as f32],
            ..Default::default()
        });

        Ok(())
    }

    fn draw(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
        #[allow(unused_variables)] frame: &demo_vk::graphics::vulkan::Frame,
    ) -> Result<()> {
        self.color_pass
            .begin_render_pass(frame, [0.0, 0.0, 0.0, 0.0]);

        self.atlas.bind_frame_descriptor(frame)?;

        self.fullscreen_quad
            .add(Sprite::default().with_tint(0.1, 0.1, 0.8, 1.0));
        self.fullscreen_quad.flush(frame)?;
        self.layer
            .begin_frame_commands(frame)?
            .draw(&self.fullscreen_quad)?
            .finish();

        self.color_pass.end_render_pass(frame);
        Ok(())
    }

    fn rebuild_swapchain_resources(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
    ) -> Result<()> {
        self.color_pass =
            SwapchainColorPass::new(gfx.vulkan.clone(), &gfx.swapchain)?;
        unsafe {
            // Safe because all frames are stalled
            self.layer
                .rebuild_pipeline(self.color_pass.renderpass(), None)?;
        }
        self.fullscreen_quad.set_viewport(gfx.swapchain.viewport());
        self.fullscreen_quad.set_scissor(gfx.swapchain.scissor());
        Ok(())
    }
}

fn main() {
    demo_main::<Sprites>();
}
