use bevy::prelude::*;

mod enemy_hit;
mod hp;

pub use hp::{hp_visualizer_system, InnerHPCircle, OuterHPCircle};

#[derive(Component)]
pub struct Particle;
