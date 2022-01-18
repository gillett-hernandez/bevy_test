use bevy::prelude::*;

use crate::{enemy::Enemy, gamestate::GameState, player::Player};

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
    pub piercing: u32,
    pub hostile: bool,
}

// need to detect and handle collisions between hostile bullets and the player, and player bullets and enemies.

pub fn player_bullet_collision_system(
    mut commands: Commands,
    mut query1: Query<(Entity, &mut Player, &Transform)>,
    query2: Query<(Entity, &Bullet, &Transform)>,
) {
    let (_, mut player, player_transform) = query1.single_mut();
    for (bullet_entity, bullet, bullet_transform) in query2.iter() {
        if !bullet.hostile {
            // skip because bullet is not hostile to player
            continue;
        }
        if (player_transform.translation - bullet_transform.translation).length_squared() < 20.0 {
            player.hp -= bullet.damage;

            // QUESTION: consider whether this should be handled as an event. i.e. fire a BulletDestroyed event so that some fx and a sound can be played.
            commands.entity(bullet_entity).despawn_recursive();
        }
    }
}

pub fn enemy_bullet_collision_system(
    mut commands: Commands,
    mut query1: Query<(Entity, &mut Enemy, &Transform)>,
    mut query2: Query<(Entity, &mut Bullet, &Transform)>,
) {
    for (_enemy_entity, mut enemy, enemy_transform) in query1.iter_mut() {
        for (bullet_entity, mut bullet, bullet_transform) in query2.iter_mut() {
            if bullet.hostile {
                // skip because bullet is hostile to player and thus not hostile to enemies
                continue;
            }
            if (enemy_transform.translation - bullet_transform.translation).length_squared() < 20.0
            {
                enemy.hp -= bullet.damage;
                if bullet.piercing == 0 {
                    // QUESTION: consider whether this should be handled as an event. i.e. fire a BulletDestroyed event so that some fx and a sound can be played.
                    commands.entity(bullet_entity).despawn_recursive();
                } else {
                    bullet.piercing -= 1;
                }
            }
        }
    }
}

pub struct BulletCollisionPlugin;
impl Plugin for BulletCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_bullet_collision_system)
                .with_system(enemy_bullet_collision_system),
        );
    }
}
