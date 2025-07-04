use barn::game::game::Game;
use crate::start_state::StartState;
use barn::game::state::State;
use barn::game::context::BarnContext;

mod start_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game = Game::new()?;
    let state: Box<dyn State<BarnContext>> = Box::new(StartState::new());
    game.run(state)?;
    
    Ok(())
}
