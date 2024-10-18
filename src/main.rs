use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;

mod rendering;
mod entities;
mod towers;
mod level;
mod settings;

use level::{Level, Point};
use settings::Settings;

struct GameState {
    level: Level,
    settings: Settings,
}

impl GameState {
    fn new() -> GameState {
        let settings = Settings::new();
        GameState {
            level: Level::new(&settings),
            settings,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw grid
        for y in 0..self.level.height {
            for x in 0..self.level.width {
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
        let path_points: Vec<Point2<f32>> = self.level.path.iter().map(|p| {
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
        for (point, color) in self.level.get_path_colors() {
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