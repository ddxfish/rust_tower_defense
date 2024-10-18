use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, Rect};

use crate::game_controller::GameController;
use crate::settings::Settings;

pub fn render_grid(
    ctx: &mut Context,
    canvas: &mut Canvas,
    game_controller: &GameController,
    settings: &Settings,
) -> GameResult {
    for y in 0..game_controller.level.height {
        for x in 0..game_controller.level.width {
            let rect = Rect::new(
                x as f32 * settings.cell_size,
                y as f32 * settings.cell_size,
                settings.cell_size,
                settings.cell_size,
            );

            let mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(1.0),
                rect,
                Color::WHITE,
            )?;

            canvas.draw(&mesh, graphics::DrawParam::default());
        }
    }
    Ok(())
}