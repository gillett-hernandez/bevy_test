use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub vertical_bounds_rotation_speed: f32,
    pub upper_bound: f32,
    pub upper_repulsion_strength: f32,
    pub lower_bound: f32,
    pub lower_repulsion_strength: f32,
}
