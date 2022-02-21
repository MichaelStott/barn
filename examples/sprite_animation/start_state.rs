
use barn::graphics::sdl_sprite::SdlSprite;
use barn::graphics::barn_gfx::BarnGFX;
use barn::graphics::color::Color;
use barn::game::state::State;
use barn::game::barn_context::BarnContext;

pub struct StartState {
    //pub sprite: SdlSprite
}

impl State<BarnContext> for StartState {
    fn update(&mut self, context: &mut BarnContext, dt: f32) -> Option<Box<dyn State<BarnContext>>> { None }

    fn draw(&mut self, context: &mut BarnContext, bgfx: &mut BarnGFX) {
        bgfx.sdl.set_draw_color(Color::SKY);
        bgfx.sdl.clear();

        bgfx.sdl.present();
    }

    fn on_enter(&mut self, context: &mut BarnContext) {
    }

    fn on_exit(&mut self, context: &mut BarnContext) {}

    fn get_name(&mut self) -> String { String::from("StartState") }
}

impl StartState {
    pub fn new() -> StartState {
        //let sprite = SdlSprite::new();
        StartState {

        }
    }
}