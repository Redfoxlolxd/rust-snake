use ggez::graphics;
use oorandom::Rand32;

use crate::{constants::{GRID_CELL_SIZE, GRID_SIZE}, direction::Direction};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
  pub x: i16,
  pub y: i16,
}

trait ModuloSigned {
  fn modulo(&self, n: Self) -> Self;
}

impl<T> ModuloSigned for T
where T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
  fn modulo(&self, n: T) -> T {
    (self.clone() % n.clone() + n.clone()) % n.clone()
  }
}

impl From<GridPosition> for graphics::Rect {
  fn from(pos: GridPosition) -> Self {
    graphics::Rect::new_i32(
      pos.x as i32 * GRID_CELL_SIZE.0 as i32,
      pos.y as i32 * GRID_CELL_SIZE.1 as i32,
      GRID_CELL_SIZE.0 as i32,
      GRID_CELL_SIZE.1 as i32,
    )
  }
}

impl From<(i16, i16)> for GridPosition {
  fn from(pos: (i16, i16)) -> Self {
    GridPosition { x: pos.0, y: pos.1 }
  }
}

impl GridPosition {
  pub fn new(x: i16, y: i16) -> Self {
    GridPosition { x, y}
  }

  pub fn random(rng: &mut Rand32, max_x: i16, max_y: i16) -> Self {
    (
      rng.rand_range(0..(max_x as u32)) as i16,
      rng.rand_range(0..(max_y as u32)) as i16,
    ).into()
  }

  pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
    match dir {
      Direction::Up => GridPosition::new(pos.x, (pos.y - 1).modulo(GRID_SIZE.1)),
      Direction::Down => GridPosition::new(pos.x, (pos.y + 1).modulo(GRID_SIZE.1)),
      Direction::Left => GridPosition::new((pos.x - 1).modulo(GRID_SIZE.0), pos.y),
      Direction::Right => GridPosition::new((pos.x + 1).modulo(GRID_SIZE.0), pos.y),
    }
  }
}
