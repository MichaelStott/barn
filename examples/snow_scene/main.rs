extern crate barn;

mod start_state;
mod snow;

use crate::barn::game::game::Game;
use crate::barn::game::context::Context;
use crate::barn::game::state::State;
use crate::start_state::StartState;
use barn::fonts::font_details::FontDetails;

fn main() {
    let mut game = Game::new(&String::from("Snow Scene"), 500, 500, false);

    let state: Box<dyn State> = Box::new(StartState::new(
    FontDetails {
            size: 32,
            path: "examples/resources/fonts/press-start/PressStart2P-vaV7.ttf"
        }
    ));
    let context = Context::new(&mut game);

    game.run(context, state).unwrap();
}
