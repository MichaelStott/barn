use barn::game::game::Game;
use crate::start_state::StartState;
use barn::game::state::State;
use barn::game::barn_context::BarnContext;
use barn::game::context::Context;

mod start_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game = Game::new("Hello World Demo", 512, 512)?;
    let state: Box<dyn State<BarnContext>> = Box::new(StartState::new());
    let context = BarnContext::new(game.get_keyboard().clone());
    game.run(state, context)?;
    
    Ok(())
}
