use bevy::prelude::*;

mod basic;

#[derive(Component)]
pub struct Enemy {
    pub hp: f32,
}

// this mirrors the player hp system to an extent.

pub fn enemy_hp_system(mut commands: Commands, query: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in query.iter() {
        if enemy.hp <= 0.0 {
            // kill enemy if hp drops <= 0
            commands.entity(entity).despawn_recursive();
        }
    }
}
