use crate::level::Point;

pub struct Enemy {
    pub position: (f32, f32),
    pub health: f32,
    pub speed: f32,
    pub path_index: usize,
    pub target: (f32, f32),
}

impl Enemy {
    pub fn new(start: Point, health: f32, speed: f32) -> Self {
        Enemy {
            position: (start.x as f32 + 0.5, start.y as f32 + 0.5),
            health,
            speed,
            path_index: 0,
            target: (start.x as f32 + 0.5, start.y as f32 + 0.5),
        }
    }

    pub fn update(&mut self, path: &[Point], delta_time: f32) {
        if self.path_index >= path.len() - 1 {
            return;
        }

        let next = &path[self.path_index + 1];
        self.target = (next.x as f32 + 0.5, next.y as f32 + 0.5);

        let dx = self.target.0 - self.position.0;
        let dy = self.target.1 - self.position.1;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < self.speed * delta_time {
            self.position = self.target;
            self.path_index += 1;
        } else {
            let move_x = dx / distance * self.speed * delta_time;
            let move_y = dy / distance * self.speed * delta_time;
            self.position.0 += move_x;
            self.position.1 += move_y;
        }
    }
}