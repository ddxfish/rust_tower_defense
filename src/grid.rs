use crate::settings::Settings;
use rand::Rng;
use ggez::graphics::Color;

#[derive(Clone, PartialEq)]
pub enum CellType {
    Empty,
    Path(Color),
    Start,
    End,
}

pub struct Grid {
    pub cells: Vec<Vec<CellType>>,
    pub path: Vec<(usize, usize)>,
}

impl Grid {
    pub fn new(settings: &Settings) -> Self {
        let mut grid = Grid {
            cells: vec![vec![CellType::Empty; settings.grid_width]; settings.grid_height],
            path: Vec::new(),
        };
        grid.generate_path(settings);
        grid
    }

    fn generate_path(&mut self, settings: &Settings) {
        let mut rng = rand::thread_rng();
        let start_x = 0;
        let start_y = rng.gen_range(0..settings.grid_height);
        
        self.path.push((start_x, start_y));
        self.cells[start_y][start_x] = CellType::Start;

        let min_path_length = (settings.grid_width + settings.grid_height) / 2;
        let max_path_length = settings.grid_width * settings.grid_height / 2;

        let target_length = rng.gen_range(min_path_length..=max_path_length);

        while self.path.len() < target_length {
            let (last_x, last_y) = *self.path.last().unwrap();
            let mut possible_moves = Vec::new();

            // Check all four directions
            for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_x = last_x as i32 + dx;
                let new_y = last_y as i32 + dy;

                if new_x >= 0 && new_x < settings.grid_width as i32 &&
                   new_y >= 0 && new_y < settings.grid_height as i32 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    if self.cells[new_y][new_x] == CellType::Empty {
                        possible_moves.push((new_x, new_y));
                    }
                }
            }

            if possible_moves.is_empty() {
                break; // Path is stuck, end it here
            }

            let next_pos = possible_moves[rng.gen_range(0..possible_moves.len())];
            self.path.push(next_pos);
            self.cells[next_pos.1][next_pos.0] = CellType::Path(self.rainbow_color(self.path.len() as f32 / target_length as f32));
        }

        // Set the last cell as the end
        let (end_x, end_y) = *self.path.last().unwrap();
        self.cells[end_y][end_x] = CellType::End;
    }

    fn rainbow_color(&self, t: f32) -> Color {
        let r = (1.0 - t) * 2.0;
        let g = 2.0 - (2.0 * t - 1.0).abs();
        let b = t * 2.0;
        Color::new(r.min(1.0).max(0.0), g.min(1.0).max(0.0), b.min(1.0).max(0.0), 1.0)
    }
}