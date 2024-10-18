use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, Rect};
use crate::game_state::GameState;
use crate::grid::CellType;
use crate::entities::{EnemyType, TowerType};

pub fn render_grid(ctx: &mut Context, state: &GameState) -> GameResult {
    for (y, row) in state.grid.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let color = match cell {
                CellType::Empty => Color::from_rgb(200, 200, 200),
                CellType::Path(color) => *color,
                CellType::Start => Color::GREEN,
                CellType::End => Color::RED,
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

pub fn render_enemies(ctx: &mut Context, state: &GameState) -> GameResult {
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

pub fn render_towers(ctx: &mut Context, state: &GameState) -> GameResult {
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