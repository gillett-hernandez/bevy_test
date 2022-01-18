use bevy::prelude::*;

use crate::{physics::Physics, player::Player, events::BulletFired};

#[derive(Component)]
pub struct Basic;

// implement some basic AI to control the physics, aiming, and bullet firing
// needs to turn towards the player if the player is in viewing range and angle
pub fn basic_ai<T: Component>(
    query: Query<(&mut Transform, &mut Physics, &T), With<Basic>>,
    event_writer: EventWriter<BulletFired<T>>,
    player: Query<&Transform, With<Player>>,
) {
}
