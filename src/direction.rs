use ggez::event::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn inverse(&self) -> Self {
    match *self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }

  pub fn from_keycode(key: KeyCode) -> Option<Direction> {
    match key {
      KeyCode::W => Some(Direction::Up),
      KeyCode::S => Some(Direction::Down),
      KeyCode::A => Some(Direction::Left),
      KeyCode::D => Some(Direction::Right),
      _ => None,
    }
  }
}