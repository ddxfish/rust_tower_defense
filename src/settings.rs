pub struct Settings {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub window_width: f32,
    pub window_height: f32,
}

impl Settings {
    pub fn new() -> Self {
        let grid_width = 40;
        let grid_height = 25;
        let cell_size = 40.0;  // Doubled from 20.0
        Settings {
            grid_width,
            grid_height,
            cell_size,
            window_width: grid_width as f32 * cell_size,
            window_height: grid_height as f32 * cell_size,
        }
    }
}