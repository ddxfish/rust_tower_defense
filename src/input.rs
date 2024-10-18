use ggez::{Context, GameResult};
use ggez::event::MouseButton;
use ggez::graphics::Rect;

use crate::game_state::GameState;
use crate::entities::{Tower, TowerType};

pub fn handle_input(ctx: &mut Context, state: &mut GameState, button: MouseButton, x: f32, y: f32) -> GameResult {
    match button {
        MouseButton::Left => handle_left_click(state, x, y),
        MouseButton::Right => handle_right_click(state),
        _ => Ok(()),
    }
}

fn handle_left_click(state: &mut GameState, x: f32, y: f32) -> GameResult {
    let grid_x = (x / state.settings.cell_size) as usize;
    let grid_y = (y / state.settings.cell_size) as usize;

    if grid_x < state.settings.grid_width && grid_y < state.settings.grid_height {
        if state.grid.cells[grid_y][grid_x] == crate::grid::CellType::Empty {
            if let Some((menu_x, menu_y)) = state.active_tower_menu {
                if let Some(tower_type) = get_selected_tower_type(x, y, menu_x, menu_y) {
                    let tower_cost = state.settings.tower_costs[tower_type as usize];
                    if state.money >= tower_cost {
                        state.towers.push(Tower::new(tower_type, (grid_x, grid_y)));
                        state.money -= tower_cost;
                        state.active_tower_menu = None;
                    }
                }
            } else {
                state.active_tower_menu = Some((x, y));
            }
        }
    } else {
        state.active_tower_menu = None;
    }

    Ok(())
}

fn handle_right_click(state: &mut GameState) -> GameResult {
    state.active_tower_menu = None;
    Ok(())
}

fn get_selected_tower_type(x: f32, y: f32, menu_x: f32, menu_y: f32) -> Option<TowerType> {
    let menu_width = 120.0;
    let button_height = 40.0;
    let button_spacing = 10.0;

    let tower_types = [TowerType::Basic, TowerType::Sniper, TowerType::Splash];

    for (i, tower_type) in tower_types.iter().enumerate() {
        let button_rect = Rect::new(
            menu_x + 10.0,
            menu_y + 10.0 + (i as f32 * (button_height + button_spacing)),
            menu_width - 20.0,
            button_height,
        );

        if button_rect.contains(ggez::mint::Point2 { x, y }) {
            return Some(*tower_type);
        }
    }

    None
}