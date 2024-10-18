use crate::settings::Settings;
use rand::Rng;
use ggez::graphics::Color;

#[derive(Clone, PartialEq)]
pub enum CellType {
    Empty,
    Path(Color),
    Waypoint,
}

pub struct Grid {
    pub cells: Vec<Vec<CellType>>,
    pub waypoints: Vec<(usize, usize)>,
    pub path: Vec<(usize, usize)>,
}

impl Grid {
    pub fn new(settings: &Settings) -> Self {
        let mut grid = Grid {
            cells: vec![vec![CellType::Empty; settings.grid_width]; settings.grid_height],
            waypoints: Vec::new(),
            path: Vec::new(),
        };
        grid.generate_waypoints(settings);
        grid.generate_path();
        grid
    }

    fn generate_waypoints(&mut self, settings: &Settings) {
        let mut rng = rand::thread_rng();
        for _ in 0..settings.num_waypoints {
            let x = rng.gen_range(0..settings.grid_width);
            let y = rng.gen_range(0..settings.grid_height);
            self.waypoints.push((x, y));
            self.cells[y][x] = CellType::Waypoint;
        }
    }

    fn generate_path(&mut self) {
        let mut path = Vec::new();
        for waypoints in self.waypoints.windows(2) {
            let (x1, y1) = waypoints[0];
            let (x2, y2) = waypoints[1];
            self.connect_points(x1, y1, x2, y2, &mut path);
        }
        
        // Color the path like a rainbow
        let total_length = path.len();
        for (i, &(x, y)) in path.iter().enumerate() {
            let color = self.rainbow_color(i as f32 / total_length as f32);
            if self.cells[y][x] != CellType::Waypoint {
                self.cells[y][x] = CellType::Path(color);
            }
        }
        
        self.path = path;
    }

    fn connect_points(&self, x1: usize, y1: usize, x2: usize, y2: usize, path: &mut Vec<(usize, usize)>) {
        let mut x = x1 as i32;
        let mut y = y1 as i32;
        let x2 = x2 as i32;
        let y2 = y2 as i32;

        while x != x2 || y != y2 {
            path.push((x as usize, y as usize));

            if x != x2 {
                x += if x < x2 { 1 } else { -1 };
            } else if y != y2 {
                y += if y < y2 { 1 } else { -1 };
            }
        }
        path.push((x2 as usize, y2 as usize));
    }

    fn rainbow_color(&self, t: f32) -> Color {
        let r = (1.0 - t) * 2.0;
        let g = 2.0 - (2.0 * t - 1.0).abs();
        let b = t * 2.0;
        Color::new(r.min(1.0).max(0.0), g.min(1.0).max(0.0), b.min(1.0).max(0.0), 1.0)
    }
}