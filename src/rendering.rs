use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, Rect, Text};
use crate::game_state::GameState;
use crate::grid::CellType;
use crate::entities::{EnemyType, TowerType};

pub fn render_game(ctx: &mut Context, state: &GameState) -> GameResult {
    graphics::clear(ctx, Color::WHITE);

    render_grid(ctx, state)?;
    render_enemies(ctx, state)?;
    render_towers(ctx, state)?;
    render_ui(ctx, state)?;

    if let Some((x, y)) = state.active_tower_menu {
        render_tower_menu(ctx, state, x, y)?;
    }

    if state.game_over {
        render_game_over(ctx, state)?;
    }

    graphics::present(ctx)?;
    Ok(())
}

fn render_grid(ctx: &mut Context, state: &GameState) -> GameResult {
    for (y, row) in state.grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = match cell {
                CellType::Empty => Color::from_rgb(200, 200, 200),
                CellType::Path(color) => *color,
                CellType::Waypoint => Color::from_rgb(100, 100, 100),
            };

            let rect = Rect::new(
                x as f32 * state.settings.cell_size,
                y as f32 * state.settings.cell_size,
                state.settings.cell_size,
                state.settings.cell_size,
            );

            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                rect,
                color,
            )?;

            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }
    }
    Ok(())
}

fn render_enemies(ctx: &mut Context, state: &GameState) -> GameResult {
    for enemy in &state.enemies {
        let color = match enemy.enemy_type {
            EnemyType::Weak => Color::from_rgb(0, 255, 0),
            EnemyType::Normal => Color::from_rgb(255, 255, 0),
            EnemyType::Strong => Color::from_rgb(255, 0, 0),
        };

        let rect = Rect::new(
            enemy.position.0 as f32 * state.settings.cell_size,
            enemy.position.1 as f32 * state.settings.cell_size,
            state.settings.cell_size,
            state.settings.cell_size,
        );

        let circle = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            ggez::mint::Point2 { x: rect.x + rect.w / 2.0, y: rect.y + rect.h / 2.0 },
            rect.w / 2.0,
            0.1,
            color,
        )?;

        graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        // Render health bar
        let health_bar_height = 5.0;
        let health_bar_width = state.settings.cell_size * (enemy.health as f32 / enemy.max_health as f32);
        let health_bar_rect = Rect::new(
            rect.x,
            rect.y - health_bar_height - 2.0,
            health_bar_width,
            health_bar_height,
        );
        let health_bar = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            health_bar_rect,
            Color::RED,
        )?;
        graphics::draw(ctx, &health_bar, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    }
    Ok(())
}

fn render_towers(ctx: &mut Context, state: &GameState) -> GameResult {
    for tower in &state.towers {
        let color = match tower.tower_type {
            TowerType::Basic => Color::from_rgb(0, 0, 255),
            TowerType::Sniper => Color::from_rgb(128, 0, 128),
            TowerType::Splash => Color::from_rgb(255, 165, 0),
        };

        let rect = Rect::new(
            tower.position.0 as f32 * state.settings.cell_size,
            tower.position.1 as f32 * state.settings.cell_size,
            state.settings.cell_size,
            state.settings.cell_size,
        );

        let tower_shape = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            rect,
            color,
        )?;

        graphics::draw(ctx, &tower_shape, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    }
    Ok(())
}

fn render_ui(ctx: &mut Context, state: &GameState) -> GameResult {
    let ui_text = Text::new(format!("Wave: {} | Money: ${} | Enemies Left: {}", 
                                    state.wave, state.money, state.enemies.len() + state.enemies_to_spawn));
    graphics::draw(ctx, &ui_text, (ggez::mint::Point2 { x: 10.0, y: 10.0 }, Color::BLACK))?;
    Ok(())
}

fn render_tower_menu(ctx: &mut Context, state: &GameState, x: f32, y: f32) -> GameResult {
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

fn render_game_over(ctx: &mut Context, state: &GameState) -> GameResult {
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