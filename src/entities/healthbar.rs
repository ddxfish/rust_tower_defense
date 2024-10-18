pub struct HealthBar {
    pub max_health: f32,
    pub current_health: f32,
    pub position: (f32, f32),
    pub width: f32,
    pub height: f32,
}

impl HealthBar {
    pub fn new(max_health: f32) -> Self {
        HealthBar {
            max_health,
            current_health: max_health,
            position: (0.0, 0.0),
            width: 40.0,
            height: 5.0,
        }
    }

    pub fn update(&mut self, current_health: f32, entity_position: (f32, f32)) {
        self.current_health = current_health;
        self.position = (
            entity_position.0 - self.width / 2.0,
            entity_position.1 - 30.0,
        );
    }

    pub fn get_fill_width(&self) -> f32 {
        (self.current_health / self.max_health) * self.width
    }
}