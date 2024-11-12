use {
    anyhow::Result,
    clap::Parser,
    core::f32,
    demo_vk::{
        app::FullscreenToggle,
        demo::{demo_main, Demo, Graphics},
        graphics::{
            ortho_projection, BindlessTextureAtlas, Sprite, SpriteLayer,
            StreamingSprites, SwapchainColorPass, TextureLoader,
        },
    },
    glfw::{Action, Key, WindowEvent},
    nalgebra::Similarity2,
    std::{sync::Arc, time::Instant},
};

#[derive(Parser, Debug)]
struct Args {}

/// A sprites demo.
struct Sprites {
    start_time: Instant,
    last_frame: Instant,
    sprites: StreamingSprites,

    fullscreen_toggle: FullscreenToggle,
    world_layer: SpriteLayer,
    atlas: BindlessTextureAtlas,
    color_pass: SwapchainColorPass,
}

impl Demo for Sprites {
    type Args = Args;

    fn new(
        window: &mut glfw::Window,
        gfx: &mut Graphics<Args>,
    ) -> Result<Self> {
        window.set_all_polling(true);
        let (w, h) = window.get_framebuffer_size();

        let ctx = &gfx.vulkan;

        let color_pass = SwapchainColorPass::new(ctx.clone(), &gfx.swapchain)?;
        let mut atlas = BindlessTextureAtlas::new(
            ctx.clone(),
            1024 * 10,
            &gfx.frames_in_flight,
        )?;

        let mut loader = TextureLoader::new(ctx.clone())?;
        let sprite_texture =
            Arc::new(loader.load_from_file("./examples/sprites/sprite.jpg")?);

        atlas.add_texture(sprite_texture);

        let world_layer = SpriteLayer::builder()
            .ctx(ctx.clone())
            .frames_in_flight(&gfx.frames_in_flight)
            .texture_atlas_layout(atlas.descriptor_set_layout())
            .render_pass(color_pass.renderpass())
            .projection(ortho_projection(w as f32 / h as f32, 10.0))
            .build()?;

        let sprites = StreamingSprites::builder()
            .ctx(ctx.clone())
            .frames_in_flight(&gfx.frames_in_flight)
            .viewport(gfx.swapchain.viewport())
            .scissor(gfx.swapchain.scissor())
            .build()?;

        Ok(Self {
            start_time: Instant::now(),
            last_frame: Instant::now(),
            fullscreen_toggle: FullscreenToggle::new(window),
            world_layer,
            sprites,
            atlas,
            color_pass,
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
        let (_dt, t) = {
            let now = Instant::now();
            let dt = now.duration_since(self.last_frame).as_secs_f32();
            self.last_frame = now;
            (dt, now.duration_since(self.start_time).as_secs_f32())
        };

        let max = 1_000;
        for i in 0..max {
            let angle = t + f32::consts::TAU * i as f32 / max as f32;
            self.sprites.add(
                Sprite::new()
                    .with_texture(0)
                    .with_sampler(0)
                    .with_similarity(&Similarity2::new(
                        [4.0 * angle.cos(), 4.0 * (angle * 3.1).sin()].into(),
                        0.0,
                        0.05,
                    )),
            );
        }

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
        self.sprites.flush(frame)?;
        self.world_layer
            .begin_frame_commands(frame)?
            .draw(&self.sprites)?
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
            self.world_layer
                .rebuild_pipeline(self.color_pass.renderpass(), None)?;
        }
        let (w, h) = window.get_framebuffer_size();
        self.world_layer
            .set_projection(&ortho_projection(w as f32 / h as f32, 10.0));
        self.sprites.set_viewport(gfx.swapchain.viewport());
        self.sprites.set_scissor(gfx.swapchain.scissor());
        Ok(())
    }
}

fn main() {
    demo_main::<Sprites>();
}
