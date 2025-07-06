extern crate barn;

mod start_state;

use barn::game::game::Game;
use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use barn::game::context::Context;
use crate::start_state::StartState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game = Game::new("Music Demo", 512, 512)?;
    let state: Box<dyn State<BarnContext>> = Box::new(StartState::new());
    let context = BarnContext::new(game.get_keyboard().clone());
    game.run(state, context)?;
    Ok(())
}
