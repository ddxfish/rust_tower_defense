use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, Rect};
use ggez::mint::Point2;

use crate::game_controller::GameController;
use crate::settings::Settings;

pub fn render_enemies(
    ctx: &mut Context,
    canvas: &mut Canvas,
    game_controller: &GameController,
    settings: &Settings,
) -> GameResult {
    for enemy in &game_controller.enemies {
        let enemy_circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2 {
                x: enemy.position.0 * settings.cell_size,
                y: enemy.position.1 * settings.cell_size,
            },
            settings.enemy_radius,
            0.1,
            Color::YELLOW,
        )?;
        canvas.draw(&enemy_circle, graphics::DrawParam::default());

        // Draw health bar background
        let health_bar_bg = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                enemy.health_bar.position.0 * settings.cell_size,
                enemy.health_bar.position.1 * settings.cell_size,
                enemy.health_bar.width * settings.cell_size,
                enemy.health_bar.height * settings.cell_size,
            ),
            Color::RED,
        )?;
        canvas.draw(&health_bar_bg, graphics::DrawParam::default());

        // Draw health bar fill
        let health_bar_fill = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                enemy.health_bar.position.0 * settings.cell_size,
                enemy.health_bar.position.1 * settings.cell_size,
                enemy.health_bar.get_fill_width() * settings.cell_size,
                enemy.health_bar.height * settings.cell_size,
            ),
            Color::GREEN,
        )?;
        canvas.draw(&health_bar_fill, graphics::DrawParam::default());
    }
    Ok(())
}