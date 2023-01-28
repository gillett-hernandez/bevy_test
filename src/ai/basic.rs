use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{body_type_stats::PlaneMovementStats, input::Intent, physics::Physics, player::Player};

use super::{AIType, AI};

// implement some basic AI to control the physics, aiming, and bullet firing
// needs to turn towards the player if the player is in viewing range and angle
pub fn plane_ai(
    time: Res<Time>,
    mut query: Query<(&mut Intent, &mut Transform, &mut AI), Without<Player>>,
    player: Query<&Transform, With<Player>>,
) {
    // TODO: add first order player position prediction (i.e. shoot at where the player will be)
    let player_position = player.single().translation;
    for (mut intent, mut transform, mut ai) in query.iter_mut() {
        if ai.ai_type != AIType::Basic {
            continue;
        }

        let enemy_position = transform.translation;

        // let (cos, sin) = (
        //     forward.dot(direction_to_player),
        //     forward.cross(direction_to_player).length(),
        // );

        let e_to_p = player_position - enemy_position;
        let forward = transform.rotation * Vec3::Y;
        let direction_to_player = (e_to_p).normalize();

        let right = transform.rotation * Vec3::X;

        let sidedness = right.dot(direction_to_player);

        // abs angle
        let angle = forward.angle_between(direction_to_player);

        if angle < 0.5 {
            intent.fire = true;
        } else {
            intent.fire = false;
        }

        // intent.turn_intent = 0.0;
        if sidedness < 0.0 {
            intent.turn_intent = 1.0;
        } else {
            intent.turn_intent = -1.0;
        }

        if e_to_p.length_squared() > 100.0 && angle < 0.9 {
            intent.accelerate = true;
        } else {
            intent.accelerate = false;
        }
    }
}
