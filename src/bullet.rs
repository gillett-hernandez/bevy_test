use bevy::prelude::*;

use crate::{enemy::Enemy, player::Player};

#[derive(Component)]
pub struct Bullet<const HOSTILE: bool> {
    pub damage: f32,
    pub piercing: bool,
}

// need to detect and handle collisions between hostile bullets and the player, and player bullets and enemies.

pub fn player_bullet_collision_system(
    mut commands: Commands,
    mut query1: Query<(Entity, &mut Player, &Transform)>,
    query2: Query<(Entity, &Bullet<true>, &Transform)>,
) {
    let (_, mut player, player_transform) = query1.single_mut();
    for (bullet_entity, bullet, bullet_transform) in query2.iter() {
        if (player_transform.translation - bullet_transform.translation).length_squared() < 20.0 {
            player.hp -= bullet.damage;
            if !bullet.piercing {
                // QUESTION: consider whether this should be handled as an event. i.e. fire a BulletDestroyed event so that some fx and a sound can be played.
                commands.entity(bullet_entity).despawn_recursive();
            }
        }
    }
}

pub fn enemy_bullet_collision_system(
    mut commands: Commands,
    mut query1: Query<(Entity, &mut Enemy, &Transform)>,
    query2: Query<(Entity, &Bullet<true>, &Transform)>,
) {
    let (_, mut enemy, enemy_transform) = query1.single_mut();
    for (bullet_entity, bullet, bullet_transform) in query2.iter() {
        if (enemy_transform.translation - bullet_transform.translation).length_squared() < 20.0 {
            enemy.hp -= bullet.damage;
            if !bullet.piercing {
                // QUESTION: consider whether this should be handled as an event. i.e. fire a BulletDestroyed event so that some fx and a sound can be played.
                commands.entity(bullet_entity).despawn_recursive();
            }
        }
    }
}
