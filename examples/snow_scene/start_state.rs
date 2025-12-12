use barn::graphics::wgpu_renderer::WgpuRenderer;
use barn::graphics::color::Color;
use barn::graphics::wgpu_sprite::WgpuSprite;
use barn::graphics::Rect;
use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use barn::math::vector2::Vector2;
use barn::graphics::texture::Texture;
use image::GenericImageView;

use crate::snow::SnowFallLayer;

pub struct StartState {
    pub snow_layer1: Option<SnowFallLayer>,
    pub snow_layer2: Option<SnowFallLayer>,
    pub snow_layer3: Option<SnowFallLayer>,
    pub cloud_offset1: f32,
    pub cloud_offset2: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub moon_sprite_data: Option<(Rect, Rect, String)>,
    pub snow_ground_sprite_data: Option<(Rect, Rect, String)>,
    pub cloud_sprite_data: Option<(Rect, Rect, String)>,
    pub gradient_sprite_data: Option<(Rect, Rect, String)>,
}

impl State<BarnContext> for StartState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> { 
        let snow_speed = 30.0 * dt;
        if let Some(ref mut layer1) = self.snow_layer1 {
            layer1.update(snow_speed);
        }
        if let Some(ref mut layer2) = self.snow_layer2 {
            layer2.update(snow_speed);
        }
        if let Some(ref mut layer3) = self.snow_layer3 {
            layer3.update(snow_speed);
        }

        let speed = 10.0 * dt;
        self.cloud_offset1 += speed;
        self.cloud_offset2 += speed;
        if self.cloud_offset1 > self.window_width {
            self.cloud_offset1 -= self.window_width + 1.0;
        }
        if self.cloud_offset2 > self.window_width {
            self.cloud_offset2 -= self.window_width + 1.0;
        }
        None
    }

    fn render(&mut self, context: &mut BarnContext, renderer: &mut WgpuRenderer) {
        // Clear rectangles and sprites from previous frame
        renderer.clear_draw_commands();
        
        // Set clear color to black
        renderer.set_clear_color(0.0, 0.0, 0.0, 1.0);
        
        // Update window dimensions if they changed
        let current_width = renderer.size.width as f32;
        let current_height = renderer.size.height as f32;
        
        if current_width != self.window_width || current_height != self.window_height {
            self.window_width = current_width;
            self.window_height = current_height;
            self.create_snow_layers();
        }
        
        // Load ground texture
        if self.snow_ground_sprite_data.is_none() {
            if let Ok(()) = renderer.load_texture("examples/resources/images/snow_ground_1.png") {
                if let Some(tex) = renderer.textures.get("examples/resources/images/snow_ground_1.png") {
                    self.snow_ground_sprite_data = Some(((0.0, 0.0, 512.0, 32.0), (0.0, current_height - 64.0, current_width, 64.0), "examples/resources/images/snow_ground_1.png".to_string()));
                }
            }
        }
        
        // Load cloud texture
        if self.cloud_sprite_data.is_none() {
            if let Ok(()) = renderer.load_texture("examples/resources/images/cloud2.png") {
                if let Some(tex) = renderer.textures.get("examples/resources/images/cloud2.png") {
                    self.cloud_sprite_data = Some(((0.0, 0.0, 256.0, 64.0), (0.0, 0.0, 256.0, 64.0), "examples/resources/images/cloud2.png".to_string()));
                }
            }
        }
        
        // Load moon texture
        if self.moon_sprite_data.is_none() {
            if let Ok(()) = renderer.load_texture("examples/resources/images/moon.png") {
                if let Some(tex) = renderer.textures.get("examples/resources/images/moon.png") {
                    self.moon_sprite_data = Some(((0.0, 0.0, 128.0, 128.0), (1000.0, 48.0, 128.0, 128.0), "examples/resources/images/moon.png".to_string()));
                }
            }
        }
        
        // Load gradient texture
        if self.gradient_sprite_data.is_none() {
            if let Ok(()) = renderer.load_texture("examples/resources/images/evening_gradient.png") {
                if let Some(tex) = renderer.textures.get("examples/resources/images/evening_gradient.png") {
                    self.gradient_sprite_data = Some(((0.0, 0.0, 1.0, 512.0), (0.0, 0.0, 512.0, 512.0), "examples/resources/images/evening_gradient.png".to_string()));
                }
            }
        }
        
        // Render background gradient first
        if let Some((src, _dst, texture_path)) = &self.gradient_sprite_data {
            let tex_size = renderer.textures.get(texture_path).map(|tex| (tex.size.width, tex.size.height));
            if let Some(tex_size) = tex_size {
                let dst = (0.0, 0.0, current_width, current_height);
                let sprite = WgpuSprite::new(*src, dst, [1.0, 1.0, 1.0, 1.0], &renderer.device, texture_path.clone(), tex_size, renderer.size.width as f32, renderer.size.height as f32);
                renderer.draw_sprite(sprite);
            }
        }
        
        // Render moon
        if let Some((src, dst, texture_path)) = &self.moon_sprite_data {
            let tex_size = renderer.textures.get(texture_path).map(|tex| (tex.size.width, tex.size.height));
            if let Some(tex_size) = tex_size {
                let sprite = WgpuSprite::new(*src, *dst, [1.0, 1.0, 1.0, 1.0], &renderer.device, texture_path.clone(), tex_size, renderer.size.width as f32, renderer.size.height as f32);
                renderer.draw_sprite(sprite);
            }
        }
        
        // Render ground
        if let Some((src, _dst, texture_path)) = &self.snow_ground_sprite_data {
            let tex_size = renderer.textures.get(texture_path).map(|tex| (tex.size.width, tex.size.height));
            if let Some(tex_size) = tex_size {
                // Calculate ground height (use the source height, or scale it proportionally)
                let ground_height = src.3; // Use the height from the source rectangle
                // Position at the bottom of the screen, spanning full width
                let dst = (0.0, current_height - ground_height, current_width, ground_height);
                let sprite = WgpuSprite::new(*src, dst, [1.0, 1.0, 1.0, 1.0], &renderer.device, texture_path.clone(), tex_size, renderer.size.width as f32, renderer.size.height as f32);
                renderer.draw_sprite(sprite);
            }
        }
        
        // Render clouds
        if let Some((src, _dst, texture_path)) = &self.cloud_sprite_data {
            let tex_size = renderer.textures.get(texture_path).map(|tex| (tex.size.width, tex.size.height));
            if let Some(tex_size) = tex_size {
                let dst_y = 30.0;
                let dst_h = 128.0;
                let dst_w = current_width;
                let cloud_color = [1.0, 1.0, 1.0, 0.25];

                // Cloud 1
                let dst1 = (self.cloud_offset1, dst_y, dst_w, dst_h);
                let sprite1 = WgpuSprite::new(*src, dst1, cloud_color, &renderer.device, texture_path.clone(), tex_size, renderer.size.width as f32, renderer.size.height as f32);
                renderer.draw_sprite(sprite1);

                // Cloud 2
                let dst2 = (self.cloud_offset2, 140.0, dst_w, dst_h);
                let sprite2 = WgpuSprite::new(*src, dst2, cloud_color, &renderer.device, texture_path.clone(), tex_size, renderer.size.width as f32, renderer.size.height as f32);
                renderer.draw_sprite(sprite2);
            }
        }
        
        // Draw snow particles as rectangles
        if let Some(ref layer3) = self.snow_layer3 {
            for snow in layer3.snow.iter() {
                let color = [
                    layer3.color.r as f32,
                    layer3.color.g as f32,
                    layer3.color.b as f32,
                    layer3.color.a as f32,
                ];
                renderer.draw_rect(snow.pos.x as i32, snow.pos.y as i32, 1, 1, color);
            }
        }
        if let Some(ref layer2) = self.snow_layer2 {
            for snow in layer2.snow.iter() {
                let color = [
                    layer2.color.r as f32,
                    layer2.color.g as f32,
                    layer2.color.b as f32,
                    layer2.color.a as f32,
                ];
                renderer.draw_rect(snow.pos.x as i32, snow.pos.y as i32, 2, 2, color);
            }
        }
        if let Some(ref layer1) = self.snow_layer1 {
            for snow in layer1.snow.iter() {
                let color = [
                    layer1.color.r as f32,
                    layer1.color.g as f32,
                    layer1.color.b as f32,
                    layer1.color.a as f32,
                ];
                renderer.draw_rect(snow.pos.x as i32, snow.pos.y as i32, 3, 3, color);
            }
        }
    }

    fn on_enter(&mut self, context: &mut BarnContext) {
        // Preload assets if needed
    }

    fn on_exit(&mut self, context: &mut BarnContext) {}

    fn get_name(&mut self) -> String { String::from("StartState") }
}

impl StartState {
    pub fn new() -> StartState{
        StartState {
            snow_layer1: None,
            snow_layer2: None,
            snow_layer3: None,
            cloud_offset1: -512.0,
            cloud_offset2: 512.0,
            window_width: 0.0,
            window_height: 0.0,
            moon_sprite_data: None,
            snow_ground_sprite_data: None,
            cloud_sprite_data: None,
            gradient_sprite_data: None,
        }
    }
    
    fn create_snow_layers(&mut self) {
        let dir: Vector2 = Vector2::new(1.0, 1.0).normalize();
        let dir2: Vector2 = Vector2::new(-1.0, 1.0).normalize();
        let color1: Color = Color::new(1.0, 1.0, 1.0, 1.0);
        let color2: Color = Color::new(0.75, 0.75, 0.75, 1.0);
        let color3: Color = Color::new(0.5, 0.5, 0.5, 1.0);
        
        self.snow_layer1 = Some(SnowFallLayer::new(dir, color1, 0.75, Vector2::new(15.0, 10.0), self.window_width, self.window_height));
        self.snow_layer2 = Some(SnowFallLayer::new(dir, color2, 0.5, Vector2::new(-37.4, -4.0), self.window_width, self.window_height));
        self.snow_layer3 = Some(SnowFallLayer::new(dir2, color3, 0.25, Vector2::new(0.4, -45.7), self.window_width, self.window_height));
    }
}