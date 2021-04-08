use crate::{constants::{GRID_SIZE, MILLIS_PER_UPDATE}, direction::Direction, food::Food, gridposition::GridPosition, snake::{Ate, Snake}};
use std::time::{Duration, Instant};
use ggez::{Context, GameResult, event::{self, KeyCode, KeyMods}, graphics};
use oorandom::Rand32;
use getrandom;

pub struct GameState {
  snake: Snake,
  food: Food,
  gameover: bool,
  rng: Rand32,
  last_update: Instant,
}

impl GameState {
  pub fn new() -> Self {
    let snake_pos = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();

    let mut seed: [u8; 8] = [0; 8];
    getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
    let mut rng = Rand32::new(u64::from_ne_bytes(seed));

    let food_pos = GridPosition::random(&mut rng, GRID_SIZE.0, GRID_SIZE.1);

    GameState {
      snake: Snake::new(snake_pos),
      food: Food::new(food_pos),
      gameover: false,
      rng,
      last_update: Instant::now(),
    }
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
    if !(Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE)) {
      return Ok(());
    }

    if !self.gameover {

      self.snake.update(&self.food);

      if let Some(ate) = self.snake.ate {
        match ate {
          Ate::Food => {
            let new_food_pos = GridPosition::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);
            self.food.pos = new_food_pos;
          }

          Ate::Itself => {
            self.gameover = true;
          }
        }
      }
    }

    self.last_update = Instant::now();

    Ok(())
  }

  fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
    graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());

    self.snake.draw(ctx)?;
    self.food.draw(ctx)?;

    graphics::present(ctx)?;
    
    ggez::timer::yield_now();

    Ok(())
  }

  fn key_down_event(
    &mut self,
    _ctx: &mut Context,
    keycode: KeyCode,
    _keymod: KeyMods,
    _repeat: bool,
  ) {
    if let Some(dir) = Direction::from_keycode(keycode) {
      if self.snake.dir != self.snake.last_update_dir && dir.inverse() != self.snake.dir {
        self.snake.next_dir = Some(dir);
      } else if dir.inverse() != self.snake.last_update_dir {
        self.snake.dir = dir;
      }
    }
  }
}