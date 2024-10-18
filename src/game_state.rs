use ggez::{Context, GameResult};
use ggez::event::MouseButton;
use rand::Rng;

use crate::settings::Settings;
use crate::grid::Grid;
use crate::entities::{Enemy, Tower, EnemyType, TowerType};
use crate::rendering::render_game;
use crate::input::handle_input;

pub struct GameState {
    pub settings: Settings,
    pub grid: Grid,
    pub enemies: Vec<Enemy>,
    pub towers: Vec<Tower>,
    pub active_tower_menu: Option<(f32, f32)>,
    pub money: i32,
    pub wave: usize,
    pub enemies_to_spawn: usize,
    pub spawn_timer: f32,
    pub game_over: bool,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let settings = Settings::new();
        let grid = Grid::new(&settings);
        Ok(GameState {
            settings,
            grid,
            enemies: Vec::new(),
            towers: Vec::new(),
            active_tower_menu: None,
            money: 100,
            wave: 1,
            enemies_to_spawn: 10,
            spawn_timer: 0.0,
            game_over: false,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_over {
            return Ok(());
        }

        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // Update towers
        for tower in &mut self.towers {
            tower.update(dt, &mut self.enemies, &mut self.money);
        }

        // Update enemies
        self.enemies.retain_mut(|enemy| {
            enemy.update(&self.grid, dt);
            if enemy.reached_end(&self.grid) {
                self.game_over = true;
                false
            } else if enemy.is_dead() {
                self.money += enemy.reward();
                false
            } else {
                true
            }
        });

        // Spawn enemies
        self.spawn_timer += dt;
        if self.enemies_to_spawn > 0 && self.spawn_timer >= self.settings.spawn_interval {
            self.spawn_enemy();
            self.enemies_to_spawn -= 1;
            self.spawn_timer = 0.0;
        }

        // Check for wave completion
        if self.enemies.is_empty() && self.enemies_to_spawn == 0 {
            self.start_next_wave();
        }

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        render_game(ctx, self)
    }

    pub fn handle_mouse_down(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        handle_input(ctx, self, button, x, y).unwrap();
    }

    fn spawn_enemy(&mut self) {
        let mut rng = rand::thread_rng();
        let enemy_type = match rng.gen_range(0..3) {
            0 => EnemyType::Weak,
            1 => EnemyType::Normal,
            _ => EnemyType::Strong,
        };

        let start_position = self.grid.path[0];
        self.enemies.push(Enemy::new(enemy_type, start_position, &self.settings));
    }

    fn start_next_wave(&mut self) {
        self.wave += 1;
        self.enemies_to_spawn = self.wave * 10;
        self.money += 50; // Bonus money for completing a wave
    }
}