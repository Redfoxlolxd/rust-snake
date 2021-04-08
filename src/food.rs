use ggez::{Context, GameResult, graphics::{self, mint}};

use crate::gridposition::GridPosition;

pub struct Food {
  pub pos: GridPosition
}

impl Food {
  pub fn new(pos: GridPosition) -> Self {
    Food { pos }
  }

  pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    let color = [0.0, 0.0, 1.0, 1.0].into();

    let rectangle = graphics::Mesh::new_rectangle(
      ctx,
      graphics::DrawMode::fill(),
      self.pos.into(),
      color
    )?;

    graphics::draw(ctx, &rectangle, (mint::Vector2 { x: 0.0, y: 0.0 },))
  }
}