use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::conf;

mod settings;
mod grid;
mod game_state;
mod entities;
mod rendering;
mod input;

use game_state::GameState;

struct MainState {
    game_state: GameState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let game_state = GameState::new(ctx)?;
        Ok(MainState { game_state })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game_state.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.game_state.draw(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: event::MouseButton, x: f32, y: f32) {
        self.game_state.handle_mouse_down(ctx, button, x, y);
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("tower_defense", "You")
        .window_setup(conf::WindowSetup::default().title("Tower Defense"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0));

    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}