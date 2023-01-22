use bevy::prelude::*;

use crate::{
    body_type_stats::PlaneMovementStats,
    config::GameConfig,
    enemy::HeatTracker,
    events::PlayerDeath,
    gamestate::GameState,
    input::Intent,
    misc::{VerticallyBounded, HP},
    mods::{
        body::{HeavyBody, MeleeBody, NormalBody},
        engines::{GungineEngine, NormalEngine, SuperboostEngine},
        guns::*,
        // Recalculated,
    },
    physics::Physics,
    userdata::UserData,
};

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum PlayerTimers {
//     ShootTimer,
// }

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerStats {
    pub contact_damage: f32,
    pub takes_contact_damage: bool,
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats {
            contact_damage: 1.0,
            takes_contact_damage: true,
        }
    }
}

pub fn add_player(
    mut commands: Commands,
    // stats_asset: Res<Assets<GameConfig>>,
    game_config: ResMut<GameConfig>,
    userdata: Res<UserData>,
    asset_server: Res<AssetServer>,
) {
    let mut commands = commands.spawn(SpatialBundle::default());
    let mut intermediate = commands
        .insert(Player)
        .insert(Intent::default())
        .insert(HP {
            hp: 100.0,
            max: 100.0,
            regen: 10.0,
        })
        .insert(Physics {
            mass: 100.0,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            gravity: Vec3::new(0.0, -4.0, 0.0),
            friction: 0.99,
        })
        .insert(VerticallyBounded)
        .insert(PlayerStats::default())
        .insert(PlaneMovementStats {
            acceleration: 10.0,
            turn_speed: 4.0,
        })
        .with_children(|e| {
            // add sprite as child so that it's affected by the transform of the parent
            e.spawn(SpriteBundle {
                texture: asset_server.get_handle("player.png"),
                transform: Transform {
                    scale: Vec3::splat(0.3),
                    translation: Vec3::new(0.0, 0.0, 1.0), // put on Z layer 1, above the background.
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    intermediate = match userdata.selected_build.0 {
        1 => intermediate
            .insert(GunType::SlugGun.data_from_type(asset_server.get_handle("bullet.png"))),
        2 => intermediate
            .insert(GunType::Laser.data_from_type(asset_server.get_handle("bullet.png"))),
        _ => intermediate
            .insert(GunType::MachineGun.data_from_type(asset_server.get_handle("bullet.png"))),
    };
    intermediate = match userdata.selected_build.1 {
        1 => intermediate.insert(HeavyBody::default()),
        2 => intermediate.insert(MeleeBody::default()),
        _ => intermediate.insert(NormalBody::default()),
    };
    match userdata.selected_build.2 {
        1 => {
            println!("constructing superboost engine");
            intermediate.insert(SuperboostEngine::new(
                game_config.superboost_acceleration_modifier,
                game_config.superboost_turn_speed_modifier,
            ))
        }
        2 => intermediate.insert(GungineEngine::default()),
        _ => intermediate.insert(NormalEngine::default()),
    };
}

pub fn player_movement_physics_system(
    time: Res<Time>,
    mut query: Query<(&Intent, &PlaneMovementStats, &mut Physics, &mut Transform), With<Player>>,
) {
    for (intent, stats, mut physics, mut transform) in query.iter_mut() {
        transform.rotate_z(intent.turn_intent * stats.turn_speed * time.delta_seconds());

        if intent.accelerate {
            physics.velocity += stats.acceleration * (transform.rotation * Vec3::Y);
        }
        if intent.brake {
            physics.velocity *= 0.975;
        }
    }
}

pub fn player_death_detection_system(
    // mut commands: Commands,
    mut event_writer: EventWriter<PlayerDeath>,
    query: Query<(Entity, &mut HP), With<Player>>,
) {
    for (_, hp) in query.iter() {
        if hp.hp <= 0.0 {
            // kill player if hp drops <= 0
            // commands.entity(entity).despawn_recursive();
            event_writer.send(PlayerDeath)
        }
    }
}

pub fn player_death_system(
    mut commands: Commands,
    mut heat_tracker: ResMut<HeatTracker>,
    mut game_state: ResMut<State<GameState>>,
    mut events: EventReader<PlayerDeath>,
    query: Query<(Entity, &Player)>,
) {
    for _ in events.iter() {
        // make sure this enemy has not already been despawned for some reason.

        // spawn fx for death
        // queue sound playing
        // despawn player
        println!("player died");
        commands.entity(query.single().0).despawn_recursive();

        // reset `heat`
        let _ = game_state.set(GameState::GameEnding);
        heat_tracker.reset();
        break;
    }
    // clear all playerdeath events
    // TODO: multiplayer - PlayerDeath will need to be updated to signal which player died.
    events.clear();
}
