use crate::graphics::wgpu_renderer::WgpuRenderer;

pub struct BarnGFX<'a> {
    renderer: &'a mut WgpuRenderer,
}

impl<'a> BarnGFX<'a> {
    pub fn new(renderer: &'a mut WgpuRenderer) -> Self {
        BarnGFX { renderer }
    }
    
    pub fn render(&mut self, surface: &mut wgpu::Surface) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(surface);
        Ok(())
    }
    
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, surface: &mut wgpu::Surface, config: &mut wgpu::SurfaceConfiguration) {
        self.renderer.resize(new_size, surface, config);
    }
    
    pub fn load_texture(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.renderer.load_texture(path)
    }

    pub fn draw_text(&mut self, text: &str, x: f32, y: f32, font_size: f32, color: [f32; 4]) {
        self.renderer.draw_text(text, x, y, font_size, color);
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: [f32; 4]) {
        self.renderer.draw_rect(x, y, width, height, color);
    }

    pub fn clear_rectangles(&mut self) {
        self.renderer.clear_draw_commands();
    }

    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.renderer.set_clear_color(r, g, b, a);
    }
}

