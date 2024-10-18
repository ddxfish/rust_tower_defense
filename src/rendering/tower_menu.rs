use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, Rect, Text, TextFragment};

use crate::settings::Settings;
use crate::towers::TowerType;

const MENU_WIDTH: f32 = 300.0;  // Doubled from 150.0
const MENU_HEIGHT: f32 = 180.0;  // Doubled from 120.0 and reduced by one row
const BUTTON_HEIGHT: f32 = 60.0;  // Doubled from 30.0

pub fn render_tower_menu(
    ctx: &mut Context,
    canvas: &mut Canvas,
    tower_menu_position: (usize, usize),
    settings: &Settings,
) -> GameResult {
    let menu_x = tower_menu_position.0 as f32 * settings.cell_size;
    let menu_y = tower_menu_position.1 as f32 * settings.cell_size;

    let menu_rect = Rect::new(menu_x, menu_y, MENU_WIDTH, MENU_HEIGHT);

    let menu_bg = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        menu_rect,
        Color::WHITE,
    )?;

    canvas.draw(&menu_bg, graphics::DrawParam::default());

    // Render tower options
    render_tower_option(ctx, canvas, "Gun Tower", menu_x, menu_y, Color::BLUE)?;
    render_tower_option(ctx, canvas, "Sniper Tower", menu_x, menu_y + BUTTON_HEIGHT, Color::RED)?;
    render_tower_option(ctx, canvas, "Flame Tower", menu_x, menu_y + 2.0 * BUTTON_HEIGHT, Color::YELLOW)?;

    Ok(())
}

fn render_tower_option(
    ctx: &mut Context,
    canvas: &mut Canvas,
    name: &str,
    x: f32,
    y: f32,
    color: Color,
) -> GameResult {
    let button_rect = Rect::new(x, y, MENU_WIDTH, BUTTON_HEIGHT);
    let button_bg = Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        button_rect,
        color,
    )?;
    canvas.draw(&button_bg, graphics::DrawParam::default());

    let text = Text::new(TextFragment::new(name).scale(32.0));  // Increased font size
    let text_dims = text.measure(ctx)?;
    canvas.draw(
        &text,
        graphics::DrawParam::default()
            .color(Color::BLACK)
            .dest([x + (MENU_WIDTH - text_dims.x) / 2.0, y + (BUTTON_HEIGHT - text_dims.y) / 2.0]),
    );

    Ok(())
}

pub fn get_selected_tower(x: f32, y: f32, tower_menu_position: (usize, usize), settings: &Settings) -> Option<TowerType> {
    let menu_x = tower_menu_position.0 as f32 * settings.cell_size;
    let menu_y = tower_menu_position.1 as f32 * settings.cell_size;

    if x >= menu_x && x <= menu_x + MENU_WIDTH && y >= menu_y && y <= menu_y + MENU_HEIGHT {
        let relative_y = y - menu_y;
        if relative_y < BUTTON_HEIGHT {
            Some(TowerType::Gun)
        } else if relative_y < 2.0 * BUTTON_HEIGHT {
            Some(TowerType::Sniper)
        } else if relative_y < 3.0 * BUTTON_HEIGHT {
            Some(TowerType::Flame)
        } else {
            None
        }
    } else {
        None
    }
}