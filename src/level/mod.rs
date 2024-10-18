use rand::Rng;
use crate::settings::Settings;

pub struct Level {
    pub width: usize,
    pub height: usize,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Level {
    pub fn new(settings: &Settings) -> Self {
        let mut rng = rand::thread_rng();
        let start = (
            rng.gen_range(0..settings.grid_width),
            rng.gen_range(0..settings.grid_height),
        );
        let mut end = start;
        while end == start {
            end = (
                rng.gen_range(0..settings.grid_width),
                rng.gen_range(0..settings.grid_height),
            );
        }
        Level {
            width: settings.grid_width,
            height: settings.grid_height,
            start,
            end,
        }
    }
}