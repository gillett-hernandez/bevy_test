use bevy::{
    prelude::{Asset, Resource},
    reflect::{TypePath},
};
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Resource, Default, Clone)]
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
