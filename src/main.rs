use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;
use ggez::timer;

mod rendering;
mod entities;
mod towers;
mod level;
mod settings;
mod enemy;
mod game_controller;

use level::Point;
use settings::Settings;
use game_controller::GameController;

struct GameState {
    settings: Settings,
    game_controller: GameController,
}

impl GameState {
    fn new() -> GameState {
        let settings = Settings::new();
        GameState {
            game_controller: GameController::new(&settings),
            settings,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = timer::delta(ctx).as_secs_f32();
        self.game_controller.update(&self.settings, delta_time);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw grid
        for y in 0..self.game_controller.level.height {
            for x in 0..self.game_controller.level.width {
                let rect = Rect::new(
                    x as f32 * self.settings.cell_size,
                    y as f32 * self.settings.cell_size,
                    self.settings.cell_size,
                    self.settings.cell_size,
                );

                let mesh = Mesh::new_rectangle(
                    ctx,
                    DrawMode::stroke(1.0),
                    rect,
                    Color::WHITE,
                )?;

                canvas.draw(&mesh, graphics::DrawParam::default());
            }
        }

        // Draw path
        let path_points: Vec<Point2<f32>> = self.game_controller.level.path.iter().map(|p| {
            Point2 {
                x: (p.x as f32 + 0.5) * self.settings.cell_size,
                y: (p.y as f32 + 0.5) * self.settings.cell_size,
            }
        }).collect();

        let path_mesh = Mesh::new_line(
            ctx,
            &path_points,
            self.settings.path_width,
            Color::WHITE,
        )?;
        canvas.draw(&path_mesh, graphics::DrawParam::default());

        // Draw waypoints (start, intermediate, and end)
        for (point, color) in self.game_controller.level.get_path_colors() {
            let rect = Rect::new(
                point.x as f32 * self.settings.cell_size,
                point.y as f32 * self.settings.cell_size,
                self.settings.cell_size,
                self.settings.cell_size,
            );

            let mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                rect,
                color,
            )?;

            canvas.draw(&mesh, graphics::DrawParam::default());
        }

        // Draw enemies
        for enemy in &self.game_controller.enemies {
            let enemy_circle = Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                Point2 {
                    x: enemy.position.0 * self.settings.cell_size,
                    y: enemy.position.1 * self.settings.cell_size,
                },
                self.settings.enemy_radius,
                0.1,
                Color::YELLOW,
            )?;
            canvas.draw(&enemy_circle, graphics::DrawParam::default());
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let settings = Settings::new();
    let (ctx, event_loop) = ContextBuilder::new("tower_defense", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Tower Defense"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(settings.window_width, settings.window_height))
        .build()?;

    let state = GameState::new();
    event::run(ctx, event_loop, state)
}