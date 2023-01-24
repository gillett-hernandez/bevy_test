use bevy::prelude::*;

use crate::{
    ai::{basic::plane_ai, AIType, AI},
    body_type_stats::PlaneMovementStats,
    events::{EnemyDeath, PlayerDeath},
    gamestate::GameState,
    input::Intent,
    misc::{random_in_circle, ToVec3, VerticallyBounded, HP},
    mods::guns::{GunData, GunType},
    physics::Physics,
    player::Player,
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
    *bundle.global_transform.translation_mut() = position.into();
    commands
        .spawn(bundle)
        .insert((
            AI::new(AIType::Basic),
            Intent::default(),
            PlaneMovementStats {
                acceleration: 5.0,
                turn_speed: 1.0,
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
            GunType::MachineGun.data_from_type(asset_server.get_handle("bullet.png")),
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

// wave system

#[derive(Resource)]
pub struct HeatTracker {
    // waves should spawn more frequently when heat is high.
    // waves should advance through a few archetypes, where early waves only spawn basic enemies and further waves spawn strong enemies.
    time_since_last_wave: f32,
    heat: f32,
    spawned_waves: u32,
}

impl HeatTracker {
    pub fn new() -> Self {
        HeatTracker {
            // TODO: marking this as the place where the timing for the first wave is currently implemented.
            time_since_last_wave: 55.0,
            heat: 1.0,
            spawned_waves: 0,
        }
    }
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

pub fn enemy_death_system(
    mut commands: Commands,
    mut heat_tracker: ResMut<HeatTracker>,
    mut events: EventReader<EnemyDeath>,
    query: Query<(Entity, &Enemy)>,
) {
    for event in events.iter() {
        // make sure this enemy has not already been despawned for some reason.
        if query.contains(event.entity) {
            // spawn fx for death
            // queue sound playing
            // despawn enemy
            commands.entity(event.entity).despawn_recursive();
            // handle `heat`
            heat_tracker.heat += event.heat;
        }
    }
}

pub fn wave_system(
    mut commands: Commands,
    time: Res<Time>,
    mut heat_tracker: ResMut<HeatTracker>,
    player: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let player_position = player.single(); // assumes there's only one player.
    if heat_tracker.time_since_last_wave > 60.0 / heat_tracker.heat {
        // spawn wave
        // enemies need to be relatively close to the player.

        match heat_tracker.spawned_waves {
            // 0..=4 => {
            // spawn enemies based on this archetype.
            // },
            _ => {
                for _ in 0..10 {
                    add_basic_enemy(&mut commands, &asset_server, player_position.translation);
                }
            }
        }
        heat_tracker.spawned_waves += 1;

        // reset time and "lower" heat
        heat_tracker.time_since_last_wave = 0.0;
        heat_tracker.heat -= 0.01;
        if heat_tracker.heat < 1.0 {
            heat_tracker.heat = 1.0;
        }
    } else {
        heat_tracker.time_since_last_wave += time.delta_seconds();
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HeatTracker::new()).add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(plane_ai)
                .with_system(wave_system)
                .with_system(enemy_death_system)
                .with_system(enemy_death_detection_system),
        );
    }
}
