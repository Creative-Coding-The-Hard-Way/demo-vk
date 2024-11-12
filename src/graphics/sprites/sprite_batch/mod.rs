mod streaming_sprites;

use ash::vk;

pub use self::streaming_sprites::StreamingSprites;

/// A batch of sprites in device memory.
pub trait SpriteBatch {
    /// The backing device buffer.
    fn buffer(&self) -> vk::Buffer;

    /// The number of sprites to render.
    fn count(&self) -> u32;

    /// The viewport for this batch of sprites.
    fn viewport(&self) -> vk::Viewport;

    /// The scissor region for this batch of sprites.
    fn scissor(&self) -> vk::Rect2D;
}
