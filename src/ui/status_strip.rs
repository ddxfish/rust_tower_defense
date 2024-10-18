use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, Text, TextFragment};

use crate::game_stats::GameStats;
use crate::settings::Settings;

pub fn render_status_strip(
    ctx: &mut Context,
    canvas: &mut Canvas,
    game_stats: &GameStats,
    settings: &Settings,
) -> GameResult {
    let strip_height = 40.0;
    let background = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, settings.window_width, strip_height),
        Color::new(0.2, 0.2, 0.2, 1.0),
    )?;
    canvas.draw(&background, graphics::DrawParam::default());

    let money_text = Text::new(TextFragment::new(format!("Money: ${}", game_stats.money)).scale(24.0));
    let wave_text = Text::new(TextFragment::new(format!("Wave: {}", game_stats.wave)).scale(24.0));
    let kills_text = Text::new(TextFragment::new(format!("Kills: {}", game_stats.enemies_killed)).scale(24.0));

    canvas.draw(&money_text, graphics::DrawParam::default().dest([10.0, 5.0]).color(Color::WHITE));
    canvas.draw(&wave_text, graphics::DrawParam::default().dest([settings.window_width / 2.0 - 50.0, 5.0]).color(Color::WHITE));
    canvas.draw(&kills_text, graphics::DrawParam::default().dest([settings.window_width - 150.0, 5.0]).color(Color::WHITE));

    Ok(())
}