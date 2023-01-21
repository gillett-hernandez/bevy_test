use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct PlaneMovementStats {
    pub acceleration: f32,
    pub turn_speed: f32,
}

#[derive(Component)]
pub struct BoatStats {
    pub aim_speed: f32,
    pub acceleration: f32,
    pub friction: f32,
}
