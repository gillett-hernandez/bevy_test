use bevy::{
    prelude::{Asset, Resource},
    reflect::TypePath,
};
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath, Serialize, Deserialize, Resource, Clone)]
pub struct GameConfig {
    pub vertical_bounds_rotation_speed: f32,
    pub upper_bound: f32,
    pub upper_repulsion_strength: f32,
    pub upper_hp_drain: f32,
    pub lower_bound: f32,
    pub lower_repulsion_strength: f32,
    pub lower_hp_drain: f32,
    pub superboost_acceleration_modifier: f32,
    pub superboost_turn_speed_modifier: f32,
    pub hitstun_time_ms: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            vertical_bounds_rotation_speed: 3.0,
            upper_bound: 500.0,
            upper_repulsion_strength: 8.1,
            upper_hp_drain: 7.0,
            lower_bound: -500.0,
            lower_repulsion_strength: 16.1,
            lower_hp_drain: 10.0,
            superboost_acceleration_modifier: 2.0,
            superboost_turn_speed_modifier: 0.3333,
            hitstun_time_ms: 10,
        }
    }
}
