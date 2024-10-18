use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, Rect};
use ggez::mint::Point2;

use crate::game_controller::GameController;
use crate::settings::Settings;

pub fn render_path(
    ctx: &mut Context,
    canvas: &mut Canvas,
    game_controller: &GameController,
    settings: &Settings,
) -> GameResult {
    // Draw path
    let path_points: Vec<Point2<f32>> = game_controller.level.path.iter().map(|p| {
        Point2 {
            x: (p.x as f32 + 0.5) * settings.cell_size,
            y: (p.y as f32 + 0.5) * settings.cell_size,
        }
    }).collect();

    let path_mesh = Mesh::new_line(
        ctx,
        &path_points,
        settings.path_width,
        Color::WHITE,
    )?;
    canvas.draw(&path_mesh, graphics::DrawParam::default());

    // Draw waypoints
    for (point, color) in game_controller.level.get_path_colors() {
        let rect = Rect::new(
            point.x as f32 * settings.cell_size,
            point.y as f32 * settings.cell_size,
            settings.cell_size,
            settings.cell_size,
        );

        let mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            rect,
            color,
        )?;

        canvas.draw(&mesh, graphics::DrawParam::default());
    }

    Ok(())
}