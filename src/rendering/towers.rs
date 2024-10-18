use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, DrawMode, Mesh, Rect};

use crate::game_controller::GameController;
use crate::settings::Settings;

pub fn render_towers(
    ctx: &mut Context,
    canvas: &mut Canvas,
    game_controller: &GameController,
    settings: &Settings,
) -> GameResult {
    for tower in &game_controller.towers {
        let tower_rect = Rect::new(
            tower.position.0 as f32 * settings.cell_size,
            tower.position.1 as f32 * settings.cell_size,
            settings.cell_size,
            settings.cell_size,
        );

        let tower_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            tower_rect,
            tower.color,
        )?;

        canvas.draw(&tower_mesh, graphics::DrawParam::default());
    }
    Ok(())
}