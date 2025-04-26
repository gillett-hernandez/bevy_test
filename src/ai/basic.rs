use bevy::prelude::*;

use crate::{input::Intent, player::Player};

use super::{AI, AIType};

// implement some basic AI to control the physics, aiming, and bullet firing
// needs to turn towards the player if the player is in viewing range and angle
pub fn plane_ai(
    mut query: Query<(&mut Intent, &Transform, &AI), Without<Player>>,
    player: Query<&Transform, With<Player>>,
) -> Result<(), BevyError> {
    // TODO: add first order player position prediction (i.e. shoot at where the player will be)
    let player_position = player.single()?.translation;
    for (mut intent, transform, ai) in query.iter_mut() {
        if ai.ai_type != AIType::Basic {
            continue;
        }

        let enemy_position = transform.translation;

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
    Ok(())
}
