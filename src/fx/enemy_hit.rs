use bevy::prelude::*;

use crate::events::EnemyHit;

pub fn enemy_hit_effect_system(
    _commands: Commands,
    _events: EventReader<EnemyHit>,
    _sprites: Res<Assets<Image>>,
) {
    // spawn short-lived particles
    
}
