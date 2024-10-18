use crate::entities::Grunt;
use crate::level::Level;
use crate::settings::Settings;

pub struct GameController {
    pub enemies: Vec<Grunt>,
    pub spawn_timer: f32,
    pub level: Level,
}

impl GameController {
    pub fn new(settings: &Settings) -> Self {
        GameController {
            enemies: Vec::new(),
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
    }
}