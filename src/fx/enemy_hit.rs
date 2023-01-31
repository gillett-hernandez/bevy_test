use bevy::prelude::*;

use crate::events::EnemyHit;

pub fn enemy_hit_effect_system(
    mut commands: Commands,
    mut events: EventReader<EnemyHit>,
    sprites: Res<Assets<Image>>,
) {
    // spawn short-lived particles
    
}
