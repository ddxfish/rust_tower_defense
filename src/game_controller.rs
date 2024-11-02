use crate::entities::Grunt;
use crate::level::Level;
use crate::settings::Settings;
use crate::towers::{Tower, TowerType};
use crate::game_stats::GameStats;
use std::time::Duration;

pub struct GameController {
    pub enemies: Vec<Grunt>,
    pub towers: Vec<Tower>,
    pub spawn_timer: f32,
    pub level: Level,
    pub game_stats: GameStats,
    pub total_time: Duration,
}

impl GameController {
    pub fn new(settings: &Settings) -> Self {
        GameController {
            enemies: Vec::new(),
            towers: Vec::new(),
            spawn_timer: 0.0,
            level: Level::new(settings),
            game_stats: GameStats::new(settings.initial_money),
            total_time: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self, settings: &Settings, delta_time: f32) {
        self.total_time += Duration::from_secs_f32(delta_time);

        // Update existing enemies
        for enemy in &mut self.enemies {
            enemy.update(&self.level.path, delta_time);
        }

        // Tower attacks
        self.process_tower_attacks();

        // Remove enemies that have reached the end or died
        self.remove_dead_enemies(settings);

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

    fn process_tower_attacks(&mut self) {
        let mut damage_events: Vec<(usize, f32)> = Vec::new();

        for tower in &mut self.towers {
            if tower.can_fire(self.total_time) {
                let targets = tower.find_targets(&self.enemies);
                for &target_index in &targets {
                    damage_events.push((target_index, tower.damage));
                }
                tower.last_fire_time = self.total_time;
            }
        }

        for (target_index, damage) in damage_events {
            if let Some(enemy) = self.enemies.get_mut(target_index) {
                enemy.health -= damage;
            }
        }
    }

    fn remove_dead_enemies(&mut self, settings: &Settings) {
        self.enemies.retain(|e| {
            if e.health <= 0.0 {
                self.game_stats.enemy_killed();
                self.game_stats.add_money(settings.enemy_kill_reward);
                false
            } else {
                e.path_index < self.level.path.len() - 1
            }
        });
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