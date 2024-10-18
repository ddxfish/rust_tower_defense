use crate::grid::Grid;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    Weak,
    Normal,
    Strong,
}

pub struct Enemy {
    pub enemy_type: EnemyType,
    pub position: (usize, usize),
    pub health: i32,
    pub max_health: i32,
    pub speed: f32,
    pub progress: f32,
    pub path_index: usize,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, start_position: (usize, usize)) -> Self {
        let (health, speed) = match enemy_type {
            EnemyType::Weak => (50, 1.0),
            EnemyType::Normal => (100, 0.8),
            EnemyType::Strong => (200, 0.6),
        };

        Enemy {
            enemy_type,
            position: start_position,
            health,
            max_health: health,
            speed,
            progress: 0.0,
            path_index: 0,
        }
    }

    pub fn update(&mut self, grid: &Grid) {
        self.progress += self.speed;
        while self.progress >= 1.0 && self.path_index < grid.path.len() - 1 {
            self.progress -= 1.0;
            self.path_index += 1;
            self.position = grid.path[self.path_index];
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn reached_end(&self, grid: &Grid) -> bool {
        self.path_index == grid.path.len() - 1
    }

    pub fn reward(&self) -> i32 {
        match self.enemy_type {
            EnemyType::Weak => 10,
            EnemyType::Normal => 20,
            EnemyType::Strong => 30,
        }
    }
}