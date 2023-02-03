use bevy::prelude::*;

use crate::{
    enemy::Enemy,
    events::{EnemyHit, PlayerHit},
    misc::{CollisionRadius, HP},
    player::Player,
};

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
    pub piercing: u32,
    pub hostile_to_player: bool,
}

// pub fn damage_calculator(v1: Vec3, v2: Vec3, m: f32) -> f32 {
//     let v = v1 - v2;
//     0.5 * m * v.length_squared()
// }

// need to detect and handle collisions between hostile bullets and the player, and player bullets and enemies.

pub fn player_bullet_collision_system(
    mut commands: Commands,
    mut hit_events: EventWriter<PlayerHit>,
    mut query1: Query<(&mut HP, &Transform, &CollisionRadius), With<Player>>,
    mut query2: Query<(Entity, &mut Bullet, &Transform, &CollisionRadius)>,
    // debug_timer: Res<DebugTimer>,
) {
    let (mut hp, player_tx, &player_collision_radius) = query1.single_mut();
    for (bullet_entity, mut bullet, bullet_tx, &bullet_collision_radius) in query2.iter_mut() {
        if !bullet.hostile_to_player {
            // skip because bullet is not hostile to player
            continue;
        }
        let length_squared =
            (player_tx.translation.truncate() - bullet_tx.translation.truncate()).length_squared();
        if length_squared < (*player_collision_radius + *bullet_collision_radius).powi(2) {
            hit_events.send_default();

            hp.hp -= bullet.damage;
            println!("player hp is now {}", hp.hp);

            if bullet.piercing == 0 {
                commands.entity(bullet_entity).despawn_recursive();
            } else {
                bullet.piercing -= 1;
            }

        }
    }

}

pub fn enemy_bullet_collision_system(
    mut commands: Commands,
    mut hit_events: EventWriter<EnemyHit>,
    mut query1: Query<(Entity, &mut HP, &Transform, &CollisionRadius), With<Enemy>>,
    mut query2: Query<(Entity, &mut Bullet, &Transform, &CollisionRadius)>,
) {
    for (enemy_entity, mut hp, enemy_tx, &enemy_collision_radius) in query1.iter_mut() {
        for (bullet_entity, mut bullet, bullet_tx, &bullet_collision_radius) in query2.iter_mut() {
            if bullet.hostile_to_player {
                // skip because bullet is hostile to player and thus not hostile to enemies
                continue;
            }
            let length_squared = (enemy_tx.translation.truncate()
                - bullet_tx.translation.truncate())
            .length_squared();

            if length_squared < (*bullet_collision_radius + *enemy_collision_radius).powi(2) {
                hit_events.send(EnemyHit {
                    entity: enemy_entity,
                    damage: bullet.damage,
                });
                hp.hp -= bullet.damage;
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
}

// pub struct BulletCollisionPlugin;
// impl Plugin for BulletCollisionPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system_set(
//             SystemSet::on_update(GameState::InGame)
//                 .with_system(player_bullet_collision_system)
//                 .with_system(enemy_bullet_collision_system),
//         );
//     }
// }
