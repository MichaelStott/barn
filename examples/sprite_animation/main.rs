extern crate barn;

mod start_state;

use barn::game::game::Game;
use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use barn::game::context::Context;
use crate::start_state::StartState;

fn main() {
    let game = Game::new("Sprite Animation Demo", 512, 512).expect("Failed to create game");
    let state: Box<dyn State<BarnContext>> = Box::new(StartState::new());
    let context = BarnContext::new(game.get_keyboard().clone());
    game.run(state, context).expect("Failed to run game");
}
