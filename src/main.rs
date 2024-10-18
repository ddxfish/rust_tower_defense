use ggez::{ContextBuilder, event};
use crate::game_state::GameState;
use crate::settings::Settings;
use crate::towers::TowerType;
mod rendering;
mod entities;
mod towers;
mod level;
mod settings;
mod game_controller;
mod game_state;

fn main() -> ggez::GameResult {
    let settings = Settings::new();
    let (ctx, event_loop) = ContextBuilder::new("tower_defense", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Tower Defense"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(settings.window_width, settings.window_height))
        .build()?;

    let state = GameState::new(settings);
    event::run(ctx, event_loop, state)
}