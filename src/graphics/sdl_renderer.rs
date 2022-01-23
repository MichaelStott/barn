use crate::math::vector2::Vector2;
use crate::graphics::SdlRect;
use crate::graphics::SdlTexture;
use crate::graphics::fill_type::FillType;
use crate::math::bounding_box_2d::BoundingBox2D;
use crate::graphics::color::Color;
use crate::fonts::SdlFont;
use crate::fonts::font_details::FontDetails;

use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::render::Canvas;

// Wrapper struct for SDL2-based rendering.
pub struct SDLRenderer {
    canvas: Canvas<Window>,
    draw_color: Color,
    x_offset: i32,
    y_offset: i32
}

impl SDLRenderer {
    pub fn new(canvas: Canvas<Window>) -> SDLRenderer {
        SDLRenderer {
            canvas: canvas,
            draw_color: Color::CLEAR,
            x_offset: 0,
            y_offset: 0
        }
    }

    pub fn set_draw_color(&mut self, color: Color) {
        self.draw_color = color;
        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(
            (color.r * 255.0) as u8, 
            (color.g * 255.0) as u8, 
            (color.b * 255.0) as u8,  
            (color.a * 255.0) as u8));
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn set_position_offset(&mut self, x_offset: i32, y_offset: i32) {
        self.x_offset = x_offset;
        self.y_offset = y_offset;
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, fill_type: FillType, offset: bool) {
        let mut _x = x;
        let mut _y = y;
        if offset {
            _x -= self.x_offset;
            _y -= self.y_offset;
        }
        if matches!(fill_type, FillType::LINE) {
            self.canvas.draw_rect(Rect::new(
                _x, _y, width, height
            )).unwrap();
        } else if matches!(fill_type, FillType::FILL) {
            self.canvas.fill_rect(Rect::new(
                _x, _y, width, height
            )).unwrap();
        }
    }

    pub fn draw_bounding_box(&mut self, bb: BoundingBox2D, offset: bool) {
        self.draw_rect(bb.origin.x as i32, bb.origin.y as i32, bb.width, bb.height, FillType::LINE, offset)
    }

    pub fn draw_texture(&mut self, texture: &mut SdlTexture, src_rect: Option<SdlRect>, dst_rect: Option<SdlRect>) {
        self.canvas
            .copy(&texture, src_rect, dst_rect)
            .unwrap();
    }

    pub fn draw_texture_ex(&mut self, texture: &mut SdlTexture, src_rect: Option<SdlRect>, dst_rect: Option<SdlRect>, angle: f32,
        center: Vector2, flip_horizontal: bool, flip_vertical: bool) {
        self.canvas
            .copy_ex(
                &texture, src_rect, dst_rect,
                angle as f64, Point::new(center.x as i32, center.y as i32),
                flip_horizontal, flip_vertical
            )
            .unwrap();
    }

    pub fn draw_text(&mut self, text: &str, color: Color, font: &SdlFont, font_details: FontDetails, x: f32, y: f32) {
        let texture_creator = self.canvas.texture_creator();
        let text_rend = font
            .render(text)
            .blended(sdl2::pixels::Color::RGBA(
                (color.r * 255.0) as u8, 
                (color.g * 255.0) as u8, 
                (color.b * 255.0) as u8,  
                (color.a * 255.0) as u8))
            .unwrap();
        let text_tex = texture_creator.create_texture_from_surface(&text_rend).unwrap();
        self.canvas
            .copy(
                &text_tex,
                None,
                Rect::new(
                    x as i32,
                    y as i32,
                    text_rend.size().0,
                    text_rend.size().1,
                ),
            )
            .unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn get_surface_width(&mut self) -> u32 {
        self.canvas.window().size().0
    }

    pub fn get_surface_height(&mut self) -> u32 {
        self.canvas.window().size().1
    }

    pub fn generate_texture_creator(&mut self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }
}