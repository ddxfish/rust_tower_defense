use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, Rect, Text};
use crate::game_state::GameState;
use crate::entities::TowerType;

pub fn render_ui(ctx: &mut Context, state: &GameState) -> GameResult {
    let ui_text = Text::new(format!("Wave: {} | Money: ${} | Enemies Left: {}", 
                                    state.wave, state.money, state.enemies.len() + state.enemies_to_spawn));
    graphics::draw(ctx, &ui_text, (ggez::mint::Point2 { x: 10.0, y: 10.0 }, Color::BLACK))?;
    Ok(())
}

pub fn render_tower_menu(ctx: &mut Context, state: &GameState, x: f32, y: f32) -> GameResult {
    let menu_width = 120.0;
    let menu_height = 160.0;
    let menu_rect = Rect::new(x, y, menu_width, menu_height);

    let menu_background = graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        menu_rect,
        Color::from_rgba(200, 200, 200, 200),
    )?;

    graphics::draw(ctx, &menu_background, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

    let tower_types = [TowerType::Basic, TowerType::Sniper, TowerType::Splash];
    let button_height = 40.0;

    for (i, tower_type) in tower_types.iter().enumerate() {
        let button_rect = Rect::new(
            x + 10.0,
            y + 10.0 + (i as f32 * (button_height + 10.0)),
            menu_width - 20.0,
            button_height,
        );

        let button_color = match tower_type {
            TowerType::Basic => Color::from_rgb(0, 0, 255),
            TowerType::Sniper => Color::from_rgb(128, 0, 128),
            TowerType::Splash => Color::from_rgb(255, 165, 0),
        };

        let button = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            button_rect,
            button_color,
        )?;

        graphics::draw(ctx, &button, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        let (tower_name, tower_cost) = match tower_type {
            TowerType::Basic => ("Basic", state.settings.tower_costs[0]),
            TowerType::Sniper => ("Sniper", state.settings.tower_costs[1]),
            TowerType::Splash => ("Splash", state.settings.tower_costs[2]),
        };

        let text = Text::new(format!("{} (${})", tower_name, tower_cost));
        let text_dims = text.dimensions(ctx);
        let text_pos = ggez::mint::Point2 {
            x: button_rect.x + (button_rect.w - text_dims.w as f32) / 2.0,
            y: button_rect.y + (button_rect.h - text_dims.h as f32) / 2.0,
        };

        graphics::draw(ctx, &text, (text_pos, Color::WHITE))?;
    }

    Ok(())
}

pub fn render_game_over(ctx: &mut Context, state: &GameState) -> GameResult {
    let game_over_text = Text::new(format!("Game Over! You reached Wave {}", state.wave));
    let text_dims = game_over_text.dimensions(ctx);
    let screen_width = state.settings.grid_width as f32 * state.settings.cell_size;
    let screen_height = state.settings.grid_height as f32 * state.settings.cell_size;
    let text_pos = ggez::mint::Point2 {
        x: (screen_width - text_dims.w as f32) / 2.0,
        y: (screen_height - text_dims.h as f32) / 2.0,
    };

    graphics::draw(ctx, &game_over_text, (text_pos, Color::RED))?;
    Ok(())
}

pub fn get_selected_tower_type(x: f32, y: f32, menu_x: f32, menu_y: f32) -> Option<TowerType> {
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