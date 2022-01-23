
use barn::graphics::SdlRect;
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::fill_type::FillType;
use barn::graphics::color::Color;
use barn::game::state::State;
use barn::game::context::Context;
use barn::fonts::font_details::FontDetails;
use barn::math::vector2::Vector2;

use crate::snow::SnowFallLayer;

pub struct StartState {
    pub font_details: FontDetails,
    pub snow_layer1: SnowFallLayer,
    pub snow_layer2: SnowFallLayer,
    pub snow_layer3: SnowFallLayer,
    pub cloud_offset1: f32,
    pub cloud_offset2: f32
}

impl State for StartState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> { 
        self.snow_layer1.update(1.0);
        self.snow_layer2.update(1.0);
        self.snow_layer3.update(1.0);

        let speed = 0.2;
        self.cloud_offset1 += speed;
        self.cloud_offset2 += speed;
        if self.cloud_offset1 >= 500.0 {
            self.cloud_offset1 -= 1024.0;
        }
        if self.cloud_offset2 >= 500.0 {
            self.cloud_offset2 -= 1024.0;
        }
        None
    }

    fn draw(&mut self, context: &mut Context, bgfx: &mut BarnGFX) {
        bgfx.sdl.set_draw_color(Color::BLACK);
        bgfx.sdl.clear();

        bgfx.sdl.draw_texture(context.load_texture(String::from("examples/resources/images/evening_gradient.png")), 
            None, 
            Some(SdlRect::new(0, 0, 512, 512))
        );
        let moon_tex = context.load_texture(String::from("examples/resources/images/moon.png"));
        moon_tex.set_color_mod(200, 200, 200);
        bgfx.sdl.draw_texture(moon_tex, 
            None, 
            Some(SdlRect::new(380, 48, 64, 64))
        );
        let cloud_tex = context.load_texture(String::from("examples/resources/images/cloud2.png"));
        cloud_tex.set_alpha_mod(50);
        bgfx.sdl.draw_texture(cloud_tex, 
            None, 
            Some(SdlRect::new(self.cloud_offset1 as i32, 30, 512, 64))
        );
        bgfx.sdl.draw_texture_ex(cloud_tex, 
            None, 
            Some(SdlRect::new(self.cloud_offset2 as i32, 70, 512, 64)),
            0.0, Vector2::ZERO, true, false
        );
        for snow in self.snow_layer3.snow.iter() {
            bgfx.sdl.set_draw_color(self.snow_layer3.color);
            bgfx.sdl.draw_rect(snow.pos.x as i32, snow.pos.y as i32, 2, 2, FillType::FILL, false)
        }
        for snow in self.snow_layer2.snow.iter() {
            bgfx.sdl.set_draw_color(self.snow_layer2.color);
            bgfx.sdl.draw_rect(snow.pos.x as i32, snow.pos.y as i32, 2, 2, FillType::FILL, false)
        }
        for snow in self.snow_layer1.snow.iter() {
            bgfx.sdl.set_draw_color(self.snow_layer1.color);
            bgfx.sdl.draw_rect(snow.pos.x as i32, snow.pos.y as i32, 3, 3, FillType::FILL, false)
        }

        bgfx.sdl.draw_texture(context.load_texture(String::from("examples/resources/images/snow_ground_1.png")), 
            None, 
            Some(SdlRect::new(0, 474, 512, 32))
        );

        bgfx.sdl.present();
    }

    fn on_enter(&mut self, context: &mut Context) {
        // Preload assets.
        context.load_font(self.font_details);
        context.load_texture(String::from("examples/resources/images/evening_gradient.png"));
        context.load_texture(String::from("examples/resources/images/moon.png"));
        context.load_texture(String::from("examples/resources/images/cloud2.png"));
        context.load_texture(String::from("examples/resources/images/snow_ground_1.png"));
    }

    fn on_exit(&mut self, context: &mut Context) {}

    fn get_name(&mut self) -> String { String::from("StartState") }
}

impl StartState {
    pub fn new(fd: FontDetails) -> StartState{
        let dir: Vector2 = Vector2::new(1.0, 1.0).normalize();
        let dir2: Vector2 = Vector2::new(-1.0, 1.0).normalize();
        let color1: Color = Color::new(1.0, 1.0, 1.0, 1.0);
        let color2: Color = Color::new(0.75, 0.75, 0.75, 1.0);
        let color3: Color = Color::new(0.5, 0.5, 0.5, 1.0);
        StartState {
            font_details: fd,
            snow_layer1: SnowFallLayer::new(dir, color1, 0.75, Vector2::new(15.0, 10.0)),
            snow_layer2: SnowFallLayer::new(dir, color2, 0.5, Vector2::new(-37.4, -4.0)),
            snow_layer3: SnowFallLayer::new(dir2, color3, 0.25, Vector2::new(0.4, -45.7)),
            cloud_offset1: -256.0,
            cloud_offset2: 256.0
        }
    }
}