
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::color::Color;
use barn::game::state::State;
use barn::game::context::Context;
use barn::fonts::font_details::FontDetails;

pub struct StartState {
    pub font_details: FontDetails
}

impl State for StartState {
    fn update(&mut self, context: &mut Context, dt: f32) -> Option<Box<dyn State>> { None }

    fn draw(&mut self, context: &mut Context, bgfx: &mut BarnGFX) {
        bgfx.sdl.set_draw_color(Color::SKY);
        bgfx.sdl.clear();

        let font = context.load_font(self.font_details);
        bgfx.sdl.set_draw_color(Color::BLACK);
        bgfx.sdl.draw_text("Hello World!", font,75.0,225.0);

        bgfx.sdl.present();
    }

    fn on_enter(&mut self, context: &mut Context) {
        context.load_font(self.font_details);
    }

    fn on_exit(&mut self, context: &mut Context) {}

    fn get_name(&mut self) -> String { String::from("StartState") }
}

impl StartState {
    pub fn new() -> StartState {
        StartState {
            font_details: FontDetails {
                size: 32,
                path: "examples/resources/fonts/press-start/PressStart2P-vaV7.ttf"
            }
        }
    }
}