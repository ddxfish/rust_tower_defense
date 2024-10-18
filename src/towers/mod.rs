use ggez::graphics::Color;

pub struct Tower {
    pub position: (usize, usize),
    pub tower_type: TowerType,
    pub range: f32,
    pub damage: f32,
    pub fire_rate: f32,
    pub color: Color,
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
            },
            TowerType::Sniper => Tower {
                position,
                tower_type,
                range: 3.0,
                damage: 30.0,
                fire_rate: 0.5,
                color: Color::RED,
            },
            TowerType::Flame => Tower {
                position,
                tower_type,
                range: 1.0,
                damage: 5.0,
                fire_rate: 2.0,
                color: Color::YELLOW,
            },
        }
    }
}