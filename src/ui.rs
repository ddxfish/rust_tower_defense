use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Rect, Text};
use crate::entities::TowerType;

pub struct DropdownItem {
    pub label: String,
    pub value: TowerType,
}

impl DropdownItem {
    pub fn new(label: &str, value: TowerType) -> Self {
        DropdownItem {
            label: label.to_string(),
            value,
        }
    }
}

pub struct Dropdown {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: String,
    items: Vec<DropdownItem>,
    is_open: bool,
    selected_index: Option<usize>,
}

impl Dropdown {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str, items: Vec<DropdownItem>) -> Self {
        Dropdown {
            x,
            y,
            width,
            height,
            label: label.to_string(),
            items,
            is_open: false,
            selected_index: None,
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let main_rect = Rect::new(self.x, self.y, self.width, self.height);
        let main_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), main_rect, Color::BLACK)?;
        graphics::draw(ctx, &main_mesh, graphics::DrawParam::default())?;

        let text = if let Some(index) = self.selected_index {
            &self.items[index].label
        } else {
            &self.label
        };
        let text_draw = Text::new(text.clone());
        let text_dims = text_draw.dimensions(ctx);
        let text_pos = ggez::mint::Point2 {
            x: self.x + 5.0,
            y: self.y + (self.height - text_dims.h as f32) / 2.0,
        };
        graphics::draw(ctx, &text_draw, (text_pos, Color::BLACK))?;

        if self.is_open {
            for (i, item) in self.items.iter().enumerate() {
                let item_y = self.y + self.height + i as f32 * self.height;
                let item_rect = Rect::new(self.x, item_y, self.width, self.height);
                let item_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), item_rect, Color::from_rgba(200, 200, 200, 255))?;
                graphics::draw(ctx, &item_mesh, graphics::DrawParam::default())?;

                let item_text = Text::new(item.label.clone());
                let item_text_dims = item_text.dimensions(ctx);
                let item_text_pos = ggez::mint::Point2 {
                    x: self.x + 5.0,
                    y: item_y + (self.height - item_text_dims.h as f32) / 2.0,
                };
                graphics::draw(ctx, &item_text, (item_text_pos, Color::BLACK))?;
            }
        }

        Ok(())
    }

    pub fn click(&mut self, x: f32, y: f32) -> bool {
        if self.is_open {
            for (i, _) in self.items.iter().enumerate() {
                let item_y = self.y + self.height + i as f32 * self.height;
                if x >= self.x && x <= self.x + self.width && y >= item_y && y <= item_y + self.height {
                    self.selected_index = Some(i);
                    self.is_open = false;
                    return true;
                }
            }
            self.is_open = false;
        } else if x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height {
            self.is_open = true;
            return true;
        }
        false
    }

    pub fn selected(&self) -> Option<TowerType> {
        self.selected_index.map(|index| self.items[index].value)
    }

    pub fn reset(&mut self) {
        self.selected_index = None;
        self.is_open = false;
    }
}