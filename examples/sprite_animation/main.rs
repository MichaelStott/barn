extern crate barn;

mod start_state;

use barn::game::game::Game;
use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use crate::start_state::StartState;

fn main() {
    let game = Game::new().expect("Failed to create game");
    let state: Box<dyn State<BarnContext>> = Box::new(StartState::new());
    game.run(state).expect("Failed to run game");
}
