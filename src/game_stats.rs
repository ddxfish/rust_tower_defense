pub struct GameStats {
    pub money: u32,
    pub wave: u32,
    pub enemies_killed: u32,
}

impl GameStats {
    pub fn new(initial_money: u32) -> Self {
        GameStats {
            money: initial_money,
            wave: 1,
            enemies_killed: 0,
        }
    }

    pub fn add_money(&mut self, amount: u32) {
        self.money += amount;
    }

    pub fn spend_money(&mut self, amount: u32) -> bool {
        if self.money >= amount {
            self.money -= amount;
            true
        } else {
            false
        }
    }

    pub fn next_wave(&mut self) {
        self.wave += 1;
    }

    pub fn enemy_killed(&mut self) {
        self.enemies_killed += 1;
    }
}