use ggez::graphics::Color;
use std::time::Duration;
use crate::entities::Grunt;

pub struct Tower {
    pub position: (usize, usize),
    pub tower_type: TowerType,
    pub range: f32,
    pub damage: f32,
    pub fire_rate: f32,
    pub color: Color,
    pub last_fire_time: Duration,
}

#[derive(Clone, Copy)]
pub enum TowerType {
    Gun,
    Sniper,
    Flame,
}

impl Tower {
    pub fn new(position: (usize, usize), tower_type: TowerType) -> Self {
        match tower_type {
            TowerType::Gun => Tower {
                position,
                tower_type,
                range: 1.5,
                damage: 10.0,
                fire_rate: 1.0,
                color: Color::BLUE,
                last_fire_time: Duration::from_secs(0),
            },
            TowerType::Sniper => Tower {
                position,
                tower_type,
                range: 4.5,
                damage: 50.0,
                fire_rate: 0.5,
                color: Color::RED,
                last_fire_time: Duration::from_secs(0),
            },
            TowerType::Flame => Tower {
                position,
                tower_type,
                range: 2.5,
                damage: 5.0,
                fire_rate: 2.0,
                color: Color::YELLOW,
                last_fire_time: Duration::from_secs(0),
            },
        }
    }

    pub fn can_fire(&self, current_time: Duration) -> bool {
        current_time.as_secs_f32() - self.last_fire_time.as_secs_f32() >= 1.0 / self.fire_rate
    }

    pub fn find_targets(&self, enemies: &[Grunt]) -> Vec<usize> {
        let mut targets = Vec::new();
        let tower_pos = (self.position.0 as f32 + 0.5, self.position.1 as f32 + 0.5);

        for (index, enemy) in enemies.iter().enumerate() {
            let dx = tower_pos.0 - enemy.position.0;
            let dy = tower_pos.1 - enemy.position.1;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= self.range {
                targets.push(index);
            }
        }

        targets.sort_by(|&a, &b| {
            let dist_a = (tower_pos.0 - enemies[a].position.0).powi(2) +
                         (tower_pos.1 - enemies[a].position.1).powi(2);
            let dist_b = (tower_pos.0 - enemies[b].position.0).powi(2) +
                         (tower_pos.1 - enemies[b].position.1).powi(2);
            dist_a.partial_cmp(&dist_b).unwrap()
        });

        match self.tower_type {
            TowerType::Gun | TowerType::Sniper => targets.truncate(1),
            TowerType::Flame => targets.truncate(3),
        }

        targets
    }
}