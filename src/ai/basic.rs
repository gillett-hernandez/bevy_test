use bevy::prelude::*;

use crate::{body_type_stats::PlaneMovementStats, physics::Physics, player::Player};

use super::{AIType, AI};

// implement some basic AI to control the physics, aiming, and bullet firing
// needs to turn towards the player if the player is in viewing range and angle
pub fn plane_ai(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Physics, &mut AI, &PlaneMovementStats), Without<Player>>,
    player: Query<&Transform, With<Player>>,
) {
    let player_position = player.single().translation;
    for (mut transform, mut physics, mut ai, stats) in query.iter_mut() {
        let enemy_position = transform.translation;
        if ai.ai_type != AIType::Basic {
            continue;
        }

        let e_to_p = enemy_position - player_position;
        let direction_to_player = (e_to_p).normalize();
        // let promoted = promote(direction_to_player);

        let forward = transform.rotation * Vec3::Y;

        let (cos, sin) = (
            forward.dot(direction_to_player),
            forward.cross(direction_to_player).length(),
        );
        let distance_to_player_squared = e_to_p.length_squared();

        if cos < -0.95 {
            // if enemy is mostly pointed towards the player,
            ai._should_fire_bullet = true;
        } else {
            ai._should_fire_bullet = false;
        }

        if sin < 0.0 {
            transform.rotation *= Quat::from_rotation_z(stats.turn_speed * time.delta_seconds());
        } else {
            transform.rotation *= Quat::from_rotation_z(-stats.turn_speed * time.delta_seconds());
        }

        // accelerate
        physics.velocity += transform.rotation * Vec3::Y * stats.acceleration;
    }
}
