pub struct Settings {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub num_waypoints: usize,
    pub spawn_interval: f32,
    pub tower_costs: [i32; 3],
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            grid_width: 20,
            grid_height: 15,
            cell_size: 40.0,
            num_waypoints: 5,
            spawn_interval: 1.5,
            tower_costs: [50, 100, 150], // Basic, Sniper, Splash
        }
    }
}