use crate::entities::Enemy;

#[derive(Clone, Copy, PartialEq)]
pub enum TowerType {
    Basic,
    Sniper,
    Splash,
}

pub struct Tower {
    pub tower_type: TowerType,
    pub position: (usize, usize),
    pub damage: i32,
    pub range: f32,
    pub fire_rate: f32,
    pub last_fire_time: f32,
}

impl Tower {
    pub fn new(tower_type: TowerType, position: (usize, usize)) -> Self {
        let (damage, range, fire_rate) = match tower_type {
            TowerType::Basic => (10, 3.0, 1.0),
            TowerType::Sniper => (50, 10.0, 3.0),
            TowerType::Splash => (15, 2.0, 1.5),
        };

        Tower {
            tower_type,
            position,
            damage,
            range,
            fire_rate,
            last_fire_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, enemies: &mut Vec<Enemy>, money: &mut i32) {
        self.last_fire_time += dt;
        if self.last_fire_time >= self.fire_rate {
            self.last_fire_time = 0.0;
            self.attack(enemies, money);
        }
    }

    fn attack(&self, enemies: &mut Vec<Enemy>, money: &mut i32) {
        match self.tower_type {
            TowerType::Basic | TowerType::Sniper => {
                if let Some(enemy) = self.find_target(enemies) {
                    enemy.take_damage(self.damage);
                    if enemy.is_dead() {
                        *money += enemy.reward();
                    }
                }
            }
            TowerType::Splash => {
                for enemy in enemies.iter_mut().filter(|e| self.is_in_range(e)) {
                    enemy.take_damage(self.damage);
                    if enemy.is_dead() {
                        *money += enemy.reward();
                    }
                }
            }
        }
    }

    fn find_target<'a>(&self, enemies: &'a mut Vec<Enemy>) -> Option<&'a mut Enemy> {
        enemies.iter_mut()
            .filter(|e| self.is_in_range(e))
            .min_by_key(|e| e.path_index)
    }

    fn is_in_range(&self, enemy: &Enemy) -> bool {
        let dx = self.position.0 as f32 - enemy.position.0 as f32;
        let dy = self.position.1 as f32 - enemy.position.1 as f32;
        (dx * dx + dy * dy).sqrt() <= self.range
    }
}