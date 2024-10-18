use ggez::graphics::Color;

pub struct Tower {
    pub position: (usize, usize),
    pub tower_type: TowerType,
    pub range: f32,
    pub damage: f32,
    pub fire_rate: f32,
    pub color: Color,
}

pub enum TowerType {
    Gun,
}

impl Tower {
    pub fn new(position: (usize, usize), tower_type: TowerType) -> Self {
        match tower_type {
            TowerType::Gun => Tower {
                position,
                tower_type,
                range: 1.0,
                damage: 10.0,
                fire_rate: 1.0,
                color: Color::BLUE,
            },
        }
    }
}