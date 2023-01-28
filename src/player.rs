use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    body_type_stats::PlaneMovementStats,
    config::GameConfig,
    events::PlayerDeath,
    fx::{InnerHPCircle, OuterHPCircle},
    gamestate::GameState,
    input::Intent,
    misc::{CollisionRadius, VerticallyBounded, HP},
    mods::{
        body::{BodyType, HeavyBody, MeleeBody, NormalBody},
        engines::{EngineType, GungineEngine, NormalEngine, SuperboostEngine},
        guns::*,
        // Recalculated,
    },
    physics::Physics,
    sprite::CommonSprites,
    userdata::UserData,
};

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
    common_sprites: Res<CommonSprites>,
) {
    let mut commands = commands.spawn(SpatialBundle::default());
    let mut intermediate = commands
        .insert(Player)
        .insert(Intent::default())
        .insert(HP {
            hp: 100.0,
            max: 100.0,
            regen: 20.0,
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
        .insert(CollisionRadius(10.0))
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
            let unwrapped = common_sprites.hp_circle.as_ref().unwrap();
            e.spawn(MaterialMesh2dBundle {
                mesh: unwrapped.inner_circle_mesh.clone(),
                material: unwrapped.inner_circle_material.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..default()
            })
            .insert(InnerHPCircle);

            e.spawn(MaterialMesh2dBundle {
                mesh: unwrapped.outer_circle_mesh.clone(),
                material: unwrapped.outer_circle_material.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..default()
            })
            .insert(OuterHPCircle);
        });

    intermediate = match userdata.selected_build.0 {
        GunType::MachineGun => intermediate.insert(GunData {
            scale: 0.9,
            ..GunType::MachineGun.data_from_type(asset_server.get_handle("bullet.png"))
        }),
        GunType::SlugGun => intermediate.insert(GunData {
            scale: 0.9,
            ..GunType::SlugGun.data_from_type(asset_server.get_handle("bullet.png"))
        }),
        GunType::Laser => intermediate
            .insert(GunType::Laser.data_from_type(asset_server.get_handle("bullet.png"))),
        GunType::Gungine => panic!(),
    };
    intermediate = match userdata.selected_build.1 {
        BodyType::Normal => intermediate.insert(NormalBody::default()),
        BodyType::Heavy => intermediate.insert(HeavyBody::default()),
        BodyType::Melee => intermediate.insert(MeleeBody::default()),
        BodyType::Nuke => todo!(),
        BodyType::Bomber => todo!(),
    };
    match userdata.selected_build.2 {
        EngineType::Normal => intermediate.insert(NormalEngine::default()),
        EngineType::Superboost => intermediate.insert(SuperboostEngine::new(
            game_config.superboost_acceleration_modifier,
            game_config.superboost_turn_speed_modifier,
        )),
        EngineType::Gungine => intermediate.insert(GungineEngine::default()),
        EngineType::Submarine => todo!(),
    };
}

pub fn plane_intent_movement_system(
    time: Res<Time>,
    mut query: Query<(&Intent, &PlaneMovementStats, &mut Physics, &mut Transform)>,
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
            event_writer.send(PlayerDeath)
        }
    }
}

pub fn player_death_system(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    events: EventReader<PlayerDeath>,
    query: Query<(Entity, &Player)>,
) {
    if !events.is_empty() {
        events.clear();

        // spawn fx for death
        // queue sound playing
        // despawn player
        println!("player died");
        commands.entity(query.single().0).despawn_recursive();

        // set next game state
        let _ = game_state.set(GameState::GameEnding);
    }
    // clear all playerdeath events
    // TODO: multiplayer - PlayerDeath will need to be updated to signal which player died.
}
