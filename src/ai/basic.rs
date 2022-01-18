use bevy::prelude::*;

use crate::{
    physics::{Physics, Position},
    player::Player,
};

use super::{AIType, AI};

// implement some basic AI to control the physics, aiming, and bullet firing
// needs to turn towards the player if the player is in viewing range and angle
pub fn basic_ai(
    mut query: Query<(&mut Transform, &mut Physics, &AI)>,
    player: Query<&Position, With<Player>>,
) {
    let player_position = player.single().0;
    for (transform, physics, ai) in query.iter_mut() {
        let enemy_position = Vec2::new(transform.translation.x, transform.translation.y);
        if ai.ai_type != AIType::Basic {
            continue;
        }

        let e_to_p = enemy_position - player_position;
        let direction_to_player = (e_to_p).normalize();
        let distance_to_player_squared = e_to_p.length_squared();

        // transform.rotation
    }
}
