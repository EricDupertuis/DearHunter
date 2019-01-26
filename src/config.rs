use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ForestConfig {
    pub tree_count: usize,
    pub centroid_count: usize,
    pub path_width: f32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GameConfig {
    pub forest: ForestConfig,
}
