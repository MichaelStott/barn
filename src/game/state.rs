use crate::graphics::barn_gfx::BarnGFX;
use crate::game::context::Context;

pub trait State {
    // Update game logic.
    fn update(&mut self, context: &mut Context, dt: f32) -> Option<Box<dyn State>>;

    // Render the game entities.
    fn draw(&mut self, context: &mut Context, bgfx: &mut BarnGFX);

    // Perform any initialization here.
    fn on_enter(&mut self, context: &mut Context);

    // Perform any cleanup before transitioning to the next state.
    fn on_exit(&mut self, context: &mut Context);

    // Get the state name.
    fn get_name(&mut self) -> String;
}