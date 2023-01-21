use bevy::prelude::*;

use crate::{
    enemy::Enemy, gamestate::GameState, misc::HP, physics::Physics, player::Player, DebugTimer,
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
    mut query1: Query<(Entity, &mut HP, &Physics, &Transform), With<Player>>,
    query2: Query<(Entity, &Bullet, &Physics, &Transform)>,
    debug_timer: Res<DebugTimer>,
) {
    let mut counter = 0;
    let (_, mut hp, player_physics, player_tx) = query1.single_mut();
    for (bullet_entity, bullet, bullet_physics, bullet_tx) in query2.iter() {
        counter += 1;
        if !bullet.hostile {
            // skip because bullet is not hostile to player
            continue;
        }
        if (player_tx.translation - bullet_tx.translation).length_squared() < 100.0 {
            let damage = damage_calculator(
                player_physics.velocity,
                bullet_physics.velocity,
                bullet.mass,
            );
            hp.hp -= damage;
            println!("player hp is now {}", hp.hp);

            // QUESTION: consider whether this should be handled as an event. i.e. fire a BulletDestroyed event so that some fx and a sound can be played.
            commands.entity(bullet_entity).despawn_recursive();
        }
    }
    // if debug_timer.just_finished() {
    //     println!("processed {} player + bullet combinations.", counter);
    // }
}

pub fn enemy_bullet_collision_system(
    mut commands: Commands,
    mut query1: Query<(Entity, &mut HP, &Physics, &Transform), With<Enemy>>,
    mut query2: Query<(Entity, &mut Bullet, &Physics, &Transform)>,
    // debug_timer: Res<Timer>,
) {
    // let mut counter = 0;
    for (_enemy_entity, mut hp, enemy_physics, enemy_tx) in query1.iter_mut() {
        for (bullet_entity, mut bullet, bullet_physics, bullet_tx) in query2.iter_mut() {
            // counter += 1;
            if bullet.hostile {
                // skip because bullet is hostile to player and thus not hostile to enemies
                continue;
            }
            if (enemy_tx.translation - bullet_tx.translation).length_squared() < 100.0 {
                let damage =
                    damage_calculator(enemy_physics.velocity, bullet_physics.velocity, bullet.mass);
                hp.hp -= damage;
                println!("enemy hp is now {}", hp.hp);
                if bullet.piercing == 0 {
                    // QUESTION: consider whether this should be handled as an event. i.e. fire a BulletDestroyed event so that some fx and a sound can be played.
                    commands.entity(bullet_entity).despawn_recursive();
                } else {
                    bullet.piercing -= 1;
                }
            }
        }
    }
    // if debug_timer.just_finished() {
    //     println!("processed {} enemy + bullet combinations.", counter);
    // }
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
