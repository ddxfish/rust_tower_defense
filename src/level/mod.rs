use rand::Rng;
use crate::settings::Settings;
use ggez::graphics::Color;

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Level {
    pub width: usize,
    pub height: usize,
    pub start: Point,
    pub end: Point,
    pub waypoints: Vec<Point>,
    pub path: Vec<Point>,
}

impl Level {
    pub fn new(settings: &Settings) -> Self {
        loop {
            let mut rng = rand::thread_rng();
            let start = Point {
                x: rng.gen_range(0..settings.grid_width),
                y: rng.gen_range(0..settings.grid_height),
            };
            let end = loop {
                let end = Point {
                    x: rng.gen_range(0..settings.grid_width),
                    y: rng.gen_range(0..settings.grid_height),
                };
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
                path: vec![],
            };

            if level.generate_waypoints(&mut rng, settings.num_waypoints) {
                return level;
            }
        }
    }

    fn generate_waypoints(&mut self, rng: &mut impl rand::Rng, num_waypoints: usize) -> bool {
        self.waypoints.clear();
        self.path.clear();

        let mut current = self.start;
        self.path.push(current);

        for _ in 0..num_waypoints {
            let next = loop {
                let candidate = Point {
                    x: rng.gen_range(0..self.width),
                    y: rng.gen_range(0..self.height),
                };
                if candidate != current && !self.waypoints.contains(&candidate) && candidate != self.end {
                    break candidate;
                }
            };
            self.waypoints.push(next);

            if !self.generate_path_segment(current, next) {
                return false;
            }

            current = next;
        }

        self.generate_path_segment(current, self.end)
    }

    fn generate_path_segment(&mut self, start: Point, end: Point) -> bool {
        let mut current = start;
        while current != end {
            let dx = (end.x as i32 - current.x as i32).signum();
            let dy = (end.y as i32 - current.y as i32).signum();

            if dx != 0 {
                current.x = (current.x as i32 + dx) as usize;
            } else {
                current.y = (current.y as i32 + dy) as usize;
            }

            if self.path.contains(&current) {
                return false;
            }
            self.path.push(current);
        }
        true
    }

    pub fn get_path_colors(&self) -> Vec<(Point, Color)> {
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