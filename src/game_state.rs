use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{EventHandler, MouseButton};
use ggez::timer;

use crate::settings::Settings;
use crate::game_controller::GameController;
use crate::towers::TowerType;
use crate::rendering;

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

        rendering::render_game(
            ctx,
            &mut canvas,
            &self.game_controller,
            &self.settings,
            self.tower_menu_open,
            self.tower_menu_position,
        )?;

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