extern crate barn;

mod start_state;

use barn::audio;
use barn::game::barn_context::BarnContext;

use crate::barn::game::game::Game;
use crate::barn::game::state::State;
use crate::start_state::StartState;

fn main() {
    let mut game = Game::new(&String::from("Hello World!"), 500, 500, false);

    audio::init(12);
    
    let state: Box<dyn State<BarnContext>> = Box::new(StartState::new());
    let context = BarnContext::new(&mut game);

    game.run(context, state).unwrap();
}
