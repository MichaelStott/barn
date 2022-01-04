extern crate barn;

mod start_state;

use crate::barn::game::game::Game;
use crate::barn::game::context::Context;
use crate::barn::game::state::State;
use crate::start_state::StartState;

fn main() {
    let mut game = Game::new(&String::from("Hello World!"), 500, 500, false);

    let state: Box<dyn State> = Box::new(StartState{});
    let context = Context::new(&mut game);

    game.run(context, state).unwrap();
}
