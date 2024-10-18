use crate::entities::Grunt;
use crate::level::Level;
use crate::settings::Settings;
use crate::towers::{Tower, TowerType};

pub struct GameController {
    pub enemies: Vec<Grunt>,
    pub towers: Vec<Tower>,
    pub spawn_timer: f32,
    pub level: Level,
}

impl GameController {
    pub fn new(settings: &Settings) -> Self {
        GameController {
            enemies: Vec::new(),
            towers: Vec::new(),
            spawn_timer: 0.0,
            level: Level::new(settings),
        }
    }

    pub fn update(&mut self, settings: &Settings, delta_time: f32) {
        // Update existing enemies
        for enemy in &mut self.enemies {
            enemy.update(&self.level.path, delta_time);
        }

        // Remove enemies that have reached the end
        self.enemies.retain(|e| e.path_index < self.level.path.len() - 1);

        // Spawn new enemies
        self.spawn_timer += delta_time;
        if self.spawn_timer >= settings.enemy_spawn_interval {
            self.spawn_timer = 0.0;
            self.enemies.push(Grunt::new(
                self.level.start,
                settings.enemy_health,
                settings.enemy_speed,
            ));
        }

        // TODO: Implement tower attacking logic here
    }

    pub fn add_tower(&mut self, position: (usize, usize), tower_type: TowerType) {
        if !self.is_position_on_path(position) {
            self.towers.push(Tower::new(position, tower_type));
        }
    }

    fn is_position_on_path(&self, position: (usize, usize)) -> bool {
        self.level.path.iter().any(|p| p.x == position.0 && p.y == position.1)
    }
}