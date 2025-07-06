extern crate barn;

mod start_state;

use barn::game::game::Game;
use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use crate::start_state::StartState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game = Game::new()?;
    let state: Box<dyn State<BarnContext>> = Box::new(StartState::new());
    game.run(state)?;
    Ok(())
}
