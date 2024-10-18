use crate::settings::Settings;
use rand::Rng;

#[derive(Clone, PartialEq)]
pub enum CellType {
    Empty,
    Path,
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
        
        // Update cells after generating the entire path
        for &(x, y) in &path {
            if self.cells[y][x] != CellType::Waypoint {
                self.cells[y][x] = CellType::Path;
            }
        }
        
        self.path = path;
    }

    fn connect_points(&self, x1: usize, y1: usize, x2: usize, y2: usize, path: &mut Vec<(usize, usize)>) {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1 as i32;
        let mut y = y1 as i32;

        loop {
            path.push((x as usize, y as usize));

            if x == x2 as i32 && y == y2 as i32 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
}