use serde::{Deserialize, Serialize};

use amethyst::config::Config;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ForestConfig {
    pub tree_count: usize,
    pub centroid_count: usize,
    pub path_width: f32,
}
