use rand::Rng;
use crate::settings::Settings;
use ggez::graphics::Color;

pub struct Level {
    pub width: usize,
    pub height: usize,
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub waypoints: Vec<(usize, usize)>,
}

impl Level {
    pub fn new(settings: &Settings) -> Self {
        let mut rng = rand::thread_rng();
        let start = (
            rng.gen_range(0..settings.grid_width),
            rng.gen_range(0..settings.grid_height),
        );
        let end = loop {
            let end = (
                rng.gen_range(0..settings.grid_width),
                rng.gen_range(0..settings.grid_height),
            );
            if end != start {
                break end;
            }
        };

        let mut level = Level {
            width: settings.grid_width,
            height: settings.grid_height,
            start,
            end,
            waypoints: vec![],
        };

        level.generate_waypoints(&mut rng, settings.num_waypoints);
        level
    }

    fn generate_waypoints(&mut self, rng: &mut impl rand::Rng, num_waypoints: usize) {
        let mut current = self.start;
        for _ in 0..num_waypoints {
            let next = loop {
                let candidate = (
                    rng.gen_range(0..self.width),
                    rng.gen_range(0..self.height),
                );
                if candidate != current && !self.waypoints.contains(&candidate) && candidate != self.end {
                    break candidate;
                }
            };
            self.waypoints.push(next);
            current = next;
        }
    }

    pub fn get_path_colors(&self) -> Vec<((usize, usize), Color)> {
        let total_points = 2 + self.waypoints.len(); // start, waypoints, and end
        let mut colors = vec![];

        colors.push((self.start, Color::RED));

        for (i, &waypoint) in self.waypoints.iter().enumerate() {
            let hue = (i + 1) as f32 / total_points as f32;
            colors.push((waypoint, hsv_to_rgb(hue, 1.0, 1.0)));
        }

        colors.push((self.end, Color::BLUE));

        colors
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match (h * 6.0).floor() as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Color::new((r + m) as f32, (g + m) as f32, (b + m) as f32, 1.0)
}