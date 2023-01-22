use bevy::{prelude::Resource, reflect::TypeUuid};
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize, TypeUuid, Resource, Default, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct GameConfig {
    pub vertical_bounds_rotation_speed: f32,
    pub upper_bound: f32,
    pub upper_repulsion_strength: f32,
    pub lower_bound: f32,
    pub lower_repulsion_strength: f32,
    pub superboost_acceleration_modifier: f32,
    pub superboost_turn_speed_modifier: f32,
}

// #[derive(Clone, Deserialize, Serialize)]
// pub struct Config {
//     pub vertical_bounds_rotation_speed: f32,
//     pub upper_bound: f32,
//     pub upper_repulsion_strength: f32,
//     pub lower_bound: f32,
//     pub lower_repulsion_strength: f32,
// }
