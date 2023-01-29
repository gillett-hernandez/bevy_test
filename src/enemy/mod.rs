use std::time::Duration;

use bevy::prelude::*;

use crate::{
    ai::{basic::plane_ai, AIType, AI},
    body_type_stats::PlaneMovementStats,
    events::EnemyDeath,
    gamestate::GameState,
    input::Intent,
    misc::{random_in_circle, CollisionRadius, ToVec3, VerticallyBounded, HP},
    mods::guns::{WeaponData, WeaponSubtype, WeaponType},
    physics::Physics,
};

pub mod basic;

#[derive(Component)]
pub struct Enemy {
    pub point_reward: f32,
    // pub xp_reward: f32,
    pub heat: f32, // heat contribution from this enemy
}

pub fn add_basic_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_position: Vec3,
) {
    let basic_enemy_spawn_radius = 300.0;
    let position = random_in_circle().to_vec3() * basic_enemy_spawn_radius + player_position;
    let mut bundle = SpatialBundle::default();
    bundle.transform.translation = position;
    commands
        .spawn(bundle)
        .insert((
            AI::new(AIType::Basic),
            Intent::default(),
            HP {
                hp: 100.0,
                max: 100.0,
                regen: 0.0,
            },
            PlaneMovementStats {
                acceleration: 6.0,
                turn_speed: 1.5,
            },
            Enemy {
                point_reward: 16.0,
                // xp_reward: 0.0,
                heat: 0.5,
            },
            Physics {
                mass: 50.0,
                velocity: Vec3::new(0.0, 0.0, 0.0),
                gravity: Vec3::new(0.0, -4.0, 0.0),
                friction: 0.995,
            },
            VerticallyBounded {},
            WeaponData {
                timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
                damage: 20.0,
                subtype: WeaponSubtype::BulletBased {
                    velocity: Vec3::new(0.0, 100.0, 0.0),
                    gravity: Vec3::new(0.0, -0.3, 0.0),
                    bullet_mass: 0.01,
                    friction: 1.0,
                    bullet_scale: 1.0,
                },
                lifetime: Duration::from_millis(3000),
                ..WeaponType::MachineGun
                    .data_from_type_and_handle(asset_server.get_handle("bullet.png"))
            },
            CollisionRadius(10.0),
        ))
        .with_children(|e| {
            // add sprite as child so that it's affected by the transform of the parent
            e.spawn(SpriteBundle {
                texture: asset_server.get_handle("enemy/basic_enemy.png"),
                transform: Transform {
                    scale: Vec3::splat(0.4),
                    translation: Vec3::new(0.0, 0.0, 1.0), // put on Z layer 1, above the background.
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

// this mirrors the player hp system to an extent.

pub fn enemy_death_detection_system(
    // mut commands: Commands,
    query: Query<(Entity, &mut HP, &Enemy)>,
    mut events: EventWriter<EnemyDeath>,
) {
    for (entity, hp, enemy) in query.iter() {
        if hp.hp <= 0.0 {
            // kill enemy if hp drops <= 0
            events.send(EnemyDeath {
                entity,
                heat: enemy.heat,
            });
            // commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(plane_ai)
                .with_system(enemy_death_detection_system),
        );
    }
}
