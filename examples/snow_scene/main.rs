extern crate barn;

mod start_state;
mod snow;

use crate::barn::game::game::Game;
use crate::barn::game::barn_context::BarnContext;
use crate::barn::game::context::Context;
use crate::barn::game::state::State;
use crate::start_state::StartState;

fn main() {
    let game = Game::new("Snow Scene Demo", 512, 512).expect("Failed to create game");
    let state: Box<dyn State<BarnContext>> = Box::new(StartState::new());
    let context = BarnContext::new(game.get_keyboard().clone());
    game.run(state, context).expect("Failed to run game");
}
