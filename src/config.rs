use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ForestConfig {
    pub tree_count: usize,
    pub centroid_count: usize,
    pub path_width: f32,
    pub start_zone_radius: f32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct BeastConfig {
    pub prey_speed: f32,
    pub player_detection_radius: f32,
    pub tree_detection_radius: f32,
    pub count: i32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct HunterConfig {
    pub time_to_go_home: f32,
    pub start_x: f32,
    pub start_y: f32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GameConfig {
    pub forest: ForestConfig,
    pub beast: BeastConfig,
    pub hunter: HunterConfig,
}
