mod grid;
mod path;
mod enemies;
mod towers;
pub mod tower_menu;

use ggez::{Context, GameResult};
use ggez::graphics::Canvas;

use crate::game_controller::GameController;
use crate::settings::Settings;

pub fn render_game(
    ctx: &mut Context,
    canvas: &mut Canvas,
    game_controller: &GameController,
    settings: &Settings,
    tower_menu_open: bool,
    tower_menu_position: (usize, usize),
) -> GameResult {
    grid::render_grid(ctx, canvas, game_controller, settings)?;
    path::render_path(ctx, canvas, game_controller, settings)?;
    enemies::render_enemies(ctx, canvas, game_controller, settings)?;
    towers::render_towers(ctx, canvas, game_controller, settings)?;
    
    if tower_menu_open {
        tower_menu::render_tower_menu(ctx, canvas, tower_menu_position, settings)?;
    }

    Ok(())
}