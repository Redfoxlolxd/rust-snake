use gamestate::GameState;
use ggez::{GameResult, event};

mod constants;
mod gamestate;
mod snake;
mod food;
mod direction;
mod gridposition;

use constants::SCREEN_SIZE;

fn main() -> GameResult {
  let (mut ctx, mut events_loop) = ggez::ContextBuilder::new("snake", "Gray xesquedele")
    .window_setup(ggez::conf::WindowSetup::default().title("snake"))
    .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
    .build()?;
  
  let mut state = GameState::new();

  event::run(&mut ctx, &mut events_loop, &mut state)
}
