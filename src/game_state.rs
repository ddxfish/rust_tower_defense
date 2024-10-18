use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect};
use ggez::event::{EventHandler, MouseButton};
use ggez::input::mouse;
use ggez::mint::Point2;
use ggez::timer;

use crate::settings::Settings;
use crate::game_controller::GameController;
use crate::towers::TowerType;

pub struct GameState {
    settings: Settings,
    game_controller: GameController,
    tower_menu_open: bool,
    tower_menu_position: (usize, usize),
}

impl GameState {
    pub fn new(settings: Settings) -> GameState {
        GameState {
            game_controller: GameController::new(&settings),
            settings,
            tower_menu_open: false,
            tower_menu_position: (0, 0),
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

        // Draw enemies and their health bars
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

            // Draw health bar background
            let health_bar_bg = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(
                    enemy.health_bar.position.0 * self.settings.cell_size,
                    enemy.health_bar.position.1 * self.settings.cell_size,
                    enemy.health_bar.width * self.settings.cell_size,
                    enemy.health_bar.height * self.settings.cell_size,
                ),
                Color::RED,
            )?;
            canvas.draw(&health_bar_bg, graphics::DrawParam::default());

            // Draw health bar fill
            let health_bar_fill = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(
                    enemy.health_bar.position.0 * self.settings.cell_size,
                    enemy.health_bar.position.1 * self.settings.cell_size,
                    enemy.health_bar.get_fill_width() * self.settings.cell_size,
                    enemy.health_bar.height * self.settings.cell_size,
                ),
                Color::GREEN,
            )?;
            canvas.draw(&health_bar_fill, graphics::DrawParam::default());
        }

        // Draw towers
        for tower in &self.game_controller.towers {
            let tower_rect = Rect::new(
                tower.position.0 as f32 * self.settings.cell_size,
                tower.position.1 as f32 * self.settings.cell_size,
                self.settings.cell_size,
                self.settings.cell_size,
            );

            let tower_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                tower_rect,
                tower.color,
            )?;

            canvas.draw(&tower_mesh, graphics::DrawParam::default());
        }

        // Draw tower menu if open
        if self.tower_menu_open {
            let menu_rect = Rect::new(
                self.tower_menu_position.0 as f32 * self.settings.cell_size,
                self.tower_menu_position.1 as f32 * self.settings.cell_size,
                self.settings.cell_size * 2.0,
                self.settings.cell_size,
            );

            let menu_bg = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                menu_rect,
                Color::WHITE,
            )?;

            canvas.draw(&menu_bg, graphics::DrawParam::default());

            let gun_tower_rect = Rect::new(
                self.tower_menu_position.0 as f32 * self.settings.cell_size,
                self.tower_menu_position.1 as f32 * self.settings.cell_size,
                self.settings.cell_size,
                self.settings.cell_size,
            );

            let gun_tower_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                gun_tower_rect,
                Color::BLUE,
            )?;

            canvas.draw(&gun_tower_mesh, graphics::DrawParam::default());
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == MouseButton::Right {
            let grid_x = (x / self.settings.cell_size) as usize;
            let grid_y = (y / self.settings.cell_size) as usize;
            self.tower_menu_open = true;
            self.tower_menu_position = (grid_x, grid_y);
        } else if button == MouseButton::Left && self.tower_menu_open {
            let grid_x = (x / self.settings.cell_size) as usize;
            let grid_y = (y / self.settings.cell_size) as usize;
            if grid_x == self.tower_menu_position.0 && grid_y == self.tower_menu_position.1 {
                self.game_controller.add_tower(self.tower_menu_position, TowerType::Gun);
                self.tower_menu_open = false;
            }
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if button == MouseButton::Right {
            self.tower_menu_open = false;
        }
        Ok(())
    }
}