use bevy::prelude::*;

use crate::{
    enemy::Enemy,
    gamestate::GameState,
    physics::{Physics, Position},
    player::Player,
};

#[derive(Component)]
pub struct Bullet {
    pub mass: f32,
    pub piercing: u32,
    pub hostile: bool,
}

pub fn damage_calculator(v1: Vec3, v2: Vec3, m: f32) -> f32 {
    let v = v1 - v2;
    0.5 * m * v.length_squared()
}
// need to detect and handle collisions between hostile bullets and the player, and player bullets and enemies.

pub fn player_bullet_collision_system(
    mut commands: Commands,
    mut query1: Query<(Entity, &mut Player, &Physics, &Position)>,
    query2: Query<(Entity, &Bullet, &Physics, &Position)>,
) {
    let (_, mut player, player_physics, player_position) = query1.single_mut();
    for (bullet_entity, bullet, bullet_physics, bullet_position) in query2.iter() {
        if !bullet.hostile {
            // skip because bullet is not hostile to player
            continue;
        }
        if (player_position.0 - bullet_position.0).length_squared() < 20.0 {
            let damage = damage_calculator(
                player_physics.velocity,
                bullet_physics.velocity,
                bullet.mass,
            );
            player.hp -= damage;

            // QUESTION: consider whether this should be handled as an event. i.e. fire a BulletDestroyed event so that some fx and a sound can be played.
            commands.entity(bullet_entity).despawn_recursive();
        }
    }
}

pub fn enemy_bullet_collision_system(
    mut commands: Commands,
    mut query1: Query<(Entity, &mut Enemy, &Physics, &Position)>,
    mut query2: Query<(Entity, &mut Bullet, &Physics, &Position)>,
) {
    for (_enemy_entity, mut enemy, enemy_physics, enemy_position) in query1.iter_mut() {
        for (bullet_entity, mut bullet, bullet_physics, bullet_position) in query2.iter_mut() {
            if bullet.hostile {
                // skip because bullet is hostile to player and thus not hostile to enemies
                continue;
            }
            if (enemy_position.0 - bullet_position.0).length_squared() < 20.0 {
                let damage =
                    damage_calculator(enemy_physics.velocity, bullet_physics.velocity, bullet.mass);
                enemy.hp -= damage;
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
