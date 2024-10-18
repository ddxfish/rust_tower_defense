use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::MouseButton;
use rand::Rng;

use crate::settings::Settings;
use crate::grid::{Grid, CellType};
use crate::entities::{Enemy, Tower, EnemyType, TowerType};
use crate::ui::{Dropdown, DropdownItem};

pub struct GameState {
    pub settings: Settings,
    pub grid: Grid,
    pub enemies: Vec<Enemy>,
    pub towers: Vec<Tower>,
    pub money: i32,
    pub wave: usize,
    pub enemies_to_spawn: usize,
    pub spawn_timer: f32,
    pub game_over: bool,
    pub tower_dropdown: Dropdown,
    pub selected_tower: Option<TowerType>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let settings = Settings::new();
        let grid = Grid::new(&settings);
        
        let tower_items = vec![
            DropdownItem::new("Basic Tower", TowerType::Basic),
            DropdownItem::new("Sniper Tower", TowerType::Sniper),
            DropdownItem::new("Splash Tower", TowerType::Splash),
        ];
        let tower_dropdown = Dropdown::new(10.0, 10.0, 200.0, 30.0, "Select Tower", tower_items);

        Ok(GameState {
            settings,
            grid,
            enemies: Vec::new(),
            towers: Vec::new(),
            money: 100,
            wave: 1,
            enemies_to_spawn: 10,
            spawn_timer: 0.0,
            game_over: false,
            tower_dropdown,
            selected_tower: None,
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

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        // Draw grid
        for (y, row) in self.grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color = match cell {
                    CellType::Empty => Color::BLACK,
                    CellType::Path(_) => Color::YELLOW,
                    CellType::Start => Color::GREEN,
                    CellType::End => Color::RED,
                };
                let rect = graphics::Rect::new(
                    x as f32 * self.settings.cell_size,
                    y as f32 * self.settings.cell_size,
                    self.settings.cell_size,
                    self.settings.cell_size,
                );
                let cell_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    color,
                )?;
                graphics::draw(ctx, &cell_mesh, graphics::DrawParam::default())?;
            }
        }

        // Draw enemies
        for enemy in &self.enemies {
            let color = match enemy.enemy_type {
                EnemyType::Weak => Color::GREEN,
                EnemyType::Normal => Color::YELLOW,
                EnemyType::Strong => Color::RED,
            };
            let rect = graphics::Rect::new(
                enemy.position.0 as f32 * self.settings.cell_size,
                enemy.position.1 as f32 * self.settings.cell_size,
                self.settings.cell_size,
                self.settings.cell_size,
            );
            let enemy_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                color,
            )?;
            graphics::draw(ctx, &enemy_mesh, graphics::DrawParam::default())?;
        }

        // Draw towers
        for tower in &self.towers {
            let color = match tower.tower_type {
                TowerType::Basic => Color::BLUE,
                TowerType::Sniper => Color::MAGENTA,
                TowerType::Splash => Color::CYAN,
            };
            let rect = graphics::Rect::new(
                tower.position.0 as f32 * self.settings.cell_size,
                tower.position.1 as f32 * self.settings.cell_size,
                self.settings.cell_size,
                self.settings.cell_size,
            );
            let tower_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                color,
            )?;
            graphics::draw(ctx, &tower_mesh, graphics::DrawParam::default())?;
        }

        self.tower_dropdown.draw(ctx)?;

        let ui_text = graphics::Text::new(format!(
            "Wave: {} | Money: ${} | Enemies Left: {}",
            self.wave,
            self.money,
            self.enemies.len() + self.enemies_to_spawn
        ));
        graphics::draw(ctx, &ui_text, (ggez::mint::Point2 { x: 10.0, y: 50.0 }, Color::BLACK))?;

        if self.game_over {
            let game_over_text = graphics::Text::new(format!("Game Over! You reached Wave {}", self.wave));
            let screen_size = graphics::screen_coordinates(ctx);
            let text_dims = game_over_text.dimensions(ctx);
            let pos = ggez::mint::Point2 {
                x: (screen_size.w - text_dims.w as f32) / 2.0,
                y: (screen_size.h - text_dims.h as f32) / 2.0,
            };
            graphics::draw(ctx, &game_over_text, (pos, Color::RED))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    pub fn handle_mouse_down(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            if self.tower_dropdown.click(x, y) {
                if let Some(selected) = self.tower_dropdown.selected() {
                    self.selected_tower = Some(selected);
                }
            } else if let Some(tower_type) = self.selected_tower {
                let grid_x = (x / self.settings.cell_size) as usize;
                let grid_y = (y / self.settings.cell_size) as usize;
                if grid_x < self.settings.grid_width && grid_y < self.settings.grid_height {
                    if self.grid.cells[grid_y][grid_x] == CellType::Empty {
                        let tower_cost = self.settings.tower_costs[tower_type as usize];
                        if self.money >= tower_cost {
                            self.towers.push(Tower::new(tower_type, (grid_x, grid_y)));
                            self.money -= tower_cost;
                            self.selected_tower = None;
                            self.tower_dropdown.reset();
                        }
                    }
                }
            }
        }
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