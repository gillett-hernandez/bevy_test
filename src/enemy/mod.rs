use bevy::prelude::*;

use crate::{
    ai::{basic::basic_ai, AIType, AI},
    events::EnemyDeath,
    gamestate::GameState,
    gun_collection::{GunData, GunType},
    misc::{promote, random_in_circle, VerticallyBounded},
    physics::{Physics, Position},
    player::Player,
};

pub mod basic;

#[derive(Component)]
pub struct Enemy {
    pub hp: f32,
    pub heat: f32, // heat contribution from this enemy
}

pub fn add_basic_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_position: Vec2,
) {
    // commands.spawn_bundle(BasicEnemyBundle::new(Vec3::ZERO, asset_server));
    let basic_enemy_spawn_radius = 300.0;
    let position =
        promote(random_in_circle()) * basic_enemy_spawn_radius + promote(player_position);
    commands
        .spawn_bundle((
            GlobalTransform::identity(),
            Transform {
                translation: position,
                ..Default::default()
            },
            AI::new(AIType::Basic),
            Enemy {
                hp: 20.0,
                heat: 0.5,
            },
            Physics {
                velocity: Vec3::new(0.0, 0.0, 0.0),
                gravity: Vec3::new(0.0, -4.0, 0.0),
                friction: 0.99,
            },
            Position(Vec2::ZERO),
            VerticallyBounded {},
            GunType::MachineGun.data_from_type(asset_server.get_handle("bullet.png")),
        ))
        .with_children(|e| {
            // add sprite as child so that it's affected by the transform of the parent
            e.spawn_bundle(SpriteBundle {
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

pub fn enemy_hp_system(
    // mut commands: Commands,
    query: Query<(Entity, &Enemy)>,
    mut events: EventWriter<EnemyDeath>,
) {
    for (entity, enemy) in query.iter() {
        if enemy.hp <= 0.0 {
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
}

pub fn enemy_death_system(
    mut commands: Commands,
    mut heat_tracker: ResMut<HeatTracker>,
    mut events: EventReader<EnemyDeath>,
    query: Query<(&Position, &Enemy)>,
) {
    for event in events.iter() {
        // spawn fx for death
        // queue sound playing
        // despawn enemy
        commands.entity(event.entity).despawn_recursive();
        // handle heat
        heat_tracker.heat += event.heat;
    }
}

pub fn wave_system(
    mut commands: Commands,
    time: Res<Time>,
    mut heat_tracker: ResMut<HeatTracker>,
    player: Query<&Position, With<Player>>,
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
                    add_basic_enemy(&mut commands, &asset_server, player_position.0);
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
                .with_system(basic_ai)
                .with_system(wave_system)
                .with_system(enemy_death_system)
                .with_system(enemy_hp_system),
        );
    }
}
