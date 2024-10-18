use crate::entities::Grunt;
use crate::level::Level;
use crate::settings::Settings;
use crate::towers::{Tower, TowerType};
use crate::game_stats::GameStats;

pub struct GameController {
    pub enemies: Vec<Grunt>,
    pub towers: Vec<Tower>,
    pub spawn_timer: f32,
    pub level: Level,
    pub game_stats: GameStats,
}

impl GameController {
    pub fn new(settings: &Settings) -> Self {
        GameController {
            enemies: Vec::new(),
            towers: Vec::new(),
            spawn_timer: 0.0,
            level: Level::new(settings),
            game_stats: GameStats::new(settings.initial_money),
        }
    }

    pub fn update(&mut self, settings: &Settings, delta_time: f32) {
        // Update existing enemies
        for enemy in &mut self.enemies {
            enemy.update(&self.level.path, delta_time);
        }

        // Remove enemies that have reached the end or died
        self.enemies.retain(|e| {
            if e.health <= 0.0 {
                self.game_stats.enemy_killed();
                self.game_stats.add_money(settings.enemy_kill_reward);
                false
            } else {
                e.path_index < self.level.path.len() - 1
            }
        });

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

    pub fn add_tower(&mut self, position: (usize, usize), tower_type: TowerType) -> bool {
        if !self.is_position_on_path(position) {
            let tower_cost = match tower_type {
                TowerType::Gun => 100,
                TowerType::Sniper => 150,
                TowerType::Flame => 200,
            };

            if self.game_stats.spend_money(tower_cost) {
                self.towers.push(Tower::new(position, tower_type));
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_position_on_path(&self, position: (usize, usize)) -> bool {
        self.level.path.iter().any(|p| p.x == position.0 && p.y == position.1)
    }
}