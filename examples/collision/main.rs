extern crate barn;

mod start_state;
pub mod tile;
pub mod player;

use barn::game::barn_context::BarnContext;

use crate::barn::game::game::Game;
use crate::barn::game::state::State;
use crate::start_state::StartState;

fn main() {
    let mut game = Game::new(&String::from("Collision"), 500, 500, false);

    let state: Box<dyn State<BarnContext>> = Box::new(StartState::new());
    let context = BarnContext::new(&mut game);

    game.run(context, state).unwrap();
}
