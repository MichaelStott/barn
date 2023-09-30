
use barn::audio::{SdlMusic, load_music, self};
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::color::Color;
use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use barn::fonts::font_details::FontDetails;
use sdl2::mixer::Music;

pub struct StartState {
    pub font_details: FontDetails
}

impl State<BarnContext> for StartState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> { None }

    fn draw(&mut self, context: &mut BarnContext, bgfx: &mut BarnGFX) {
        bgfx.sdl.set_draw_color(Color::SKY);
        bgfx.sdl.clear();

        let font = context.load_font(self.font_details);
        bgfx.sdl.set_draw_color(Color::BLACK);
        bgfx.sdl.draw_text("Hello World!", font,250.0,250.0, 0.5, 0.5, true, true);

        bgfx.sdl.present();
    }

    fn on_enter(&mut self, context: &mut BarnContext) {
        context.load_font(self.font_details);
        let music = context.load_music("examples/resources/audio/laidback.mp3".to_string());
        music.play(0);
    }

    fn on_exit(&mut self, context: &mut BarnContext) {}

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