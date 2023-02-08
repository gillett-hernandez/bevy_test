use bevy::prelude::*;

use crate::{
    body_type_stats::PlaneMovementStats,
    config::GameConfig,
    events::PlayerDeath,
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
) {
    let bullet_image_handle = asset_server.get_handle("images/bullet.png");
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
                texture: asset_server.get_handle("images/player.png"),
                transform: Transform {
                    scale: Vec3::splat(0.3),
                    translation: Vec3::new(0.0, 0.0, 1.0), // put on Z layer 1, above the background.
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    intermediate = match userdata.selected_build.0 {
        WeaponType::MachineGun => {
            let bundle =
                WeaponType::MachineGun.data_from_type_and_handle(bullet_image_handle.clone());
            let WeaponSubtype::BulletBased { velocity, gravity, bullet_mass, friction, bullet_scale:_ } = bundle.subtype else {
                panic!();
            };
            intermediate.insert(WeaponData {
                subtype: WeaponSubtype::BulletBased {
                    bullet_scale: 0.9,
                    velocity,
                    gravity,
                    bullet_mass,
                    friction,
                },
                ..bundle
            })
        }
        WeaponType::SlugGun => {
            let bundle = WeaponType::SlugGun.data_from_type_and_handle(bullet_image_handle.clone());
            let WeaponSubtype::BulletBased { velocity, gravity, bullet_mass, friction, bullet_scale:_ } = bundle.subtype else {
                panic!();
            };
            intermediate.insert(WeaponData {
                subtype: WeaponSubtype::BulletBased {
                    bullet_scale: 0.9,
                    velocity,
                    gravity,
                    bullet_mass,
                    friction,
                },
                ..bundle
            })
        }
        WeaponType::Laser => intermediate
            .insert(WeaponType::Laser.data_from_type_and_handle(bullet_image_handle.clone())),
        WeaponType::Gungine => {
            panic!("gungine is not a selectable gun type");
        }
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
        EngineType::Gungine => {
            intermediate.insert(GungineEngine::default());
            intermediate.with_children(|e| {
                e.spawn(SpatialBundle::default())
                    .insert(WeaponType::Gungine.data_from_type_and_handle(bullet_image_handle));
            })
        }
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

pub fn player_death_system_stage_one(
    mut game_state: ResMut<State<GameState>>,
    events: EventReader<PlayerDeath>,
) {
    if !events.is_empty() {
        events.clear();

        // TODO: spawn vfx for death
        // TODO: queue player death sound playing, using another system and listening to PlayerDeath

        // set next game state
        let _ = game_state.set(GameState::GameEnding);
    }
    // clear all playerdeath events
    // TODO: multiplayer - PlayerDeath will need to be updated to signal which player died.
}

pub fn player_death_system_stage_two(
    mut commands: Commands,
    events: EventReader<PlayerDeath>,
    query: Query<(Entity, &Player)>,
) {
    // TODO: don't actually despawn the player, instead have a function `reset` that gets all the player's components and calls reset on them or something

    if !events.is_empty() && !query.is_empty() {
        // despawn player
        info!("player died");
        commands.entity(query.single().0).despawn_recursive();
    }
}
