use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, Rect};

use crate::settings::Settings;

pub fn render_tower_menu(
    ctx: &mut Context,
    canvas: &mut Canvas,
    tower_menu_position: (usize, usize),
    settings: &Settings,
) -> GameResult {
    let menu_rect = Rect::new(
        tower_menu_position.0 as f32 * settings.cell_size,
        tower_menu_position.1 as f32 * settings.cell_size,
        settings.cell_size * 2.0,
        settings.cell_size,
    );

    let menu_bg = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        menu_rect,
        Color::WHITE,
    )?;

    canvas.draw(&menu_bg, graphics::DrawParam::default());

    let gun_tower_rect = Rect::new(
        tower_menu_position.0 as f32 * settings.cell_size,
        tower_menu_position.1 as f32 * settings.cell_size,
        settings.cell_size,
        settings.cell_size,
    );

    let gun_tower_mesh = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        gun_tower_rect,
        Color::BLUE,
    )?;

    canvas.draw(&gun_tower_mesh, graphics::DrawParam::default());

    Ok(())
}