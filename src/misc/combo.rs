use bevy::prelude::*;

use crate::{enemy::Enemy, events::EnemyDeath};

pub fn combo_enemy_death_subscriber(
    mut events: EventReader<EnemyDeath>,
    query: Query<(Entity, &Enemy)>,
) {
}
