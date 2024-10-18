pub struct Settings {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub num_waypoints: usize,
    pub path_width: f32,
    pub enemy_radius: f32,
    pub enemy_speed: f32,
    pub enemy_health: f32,
    pub enemy_spawn_interval: f32,
    pub initial_money: u32,
    pub enemy_kill_reward: u32,
}

impl Settings {
    pub fn new() -> Self {
        let grid_width = 40;
        let grid_height = 25;
        let cell_size = 40.0;
        Settings {
            grid_width,
            grid_height,
            cell_size,
            window_width: grid_width as f32 * cell_size,
            window_height: grid_height as f32 * cell_size + 40.0, // Added 40.0 for status strip
            num_waypoints: 5,
            path_width: 4.0,
            enemy_radius: cell_size * 0.4,
            enemy_speed: 2.0,
            enemy_health: 100.0,
            enemy_spawn_interval: 5.0,
            initial_money: 500,
            enemy_kill_reward: 10,
        }
    }
}