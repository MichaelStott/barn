
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::fill_type::FillType;
use barn::graphics::color::Color;
use barn::game::state::State;
use barn::game::context::Context;

pub struct StartState {}

impl State for StartState {
    fn update(&mut self, context: &mut Context) -> Option<Box<dyn State>> { None }

    fn draw(&mut self, context: &mut Context, bgfx: &mut BarnGFX) {
        bgfx.sdl.set_draw_color(Color::SKY);
        bgfx.sdl.clear();

        bgfx.sdl.set_draw_color(Color::RED);
        bgfx.sdl.draw_rect(0, 0, 20, 20, FillType::FILL, true);

        bgfx.sdl.present();
    }

    fn on_enter(&mut self, context: &mut Context) {}

    fn on_exit(&mut self, context: &mut Context) {}

    fn get_name(&mut self) -> String { String::from("StartState") }
}