use bevy::prelude::*;

use crate::{physics::Physics, player::Player};

use super::{AIType, AI};

// implement some basic AI to control the physics, aiming, and bullet firing
// needs to turn towards the player if the player is in viewing range and angle
pub fn basic_ai(
    mut query: Query<(&mut Transform, &mut Physics, &AI)>,
    player: Query<&Transform, With<Player>>,
) {
    let player_translation = player.single().translation;
    for (transform, physics, ai) in query.iter_mut() {
        if ai.ai_type != AIType::Basic {
            continue;
        }
    }
}
