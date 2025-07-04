pub mod barn_gfx;
pub mod color;
pub mod fill_type;
pub mod wgpu_renderer;
pub mod wgpu_sprite;
pub mod texture;

pub type Rect = (f32, f32, f32, f32); // x, y, width, height
pub type Texture = wgpu::Texture;
pub type BlendType = wgpu::BlendState;