use bevy::prelude::*;
// use bevy::sprite::SpriteBundle;

use crate::{
    body_type_stats::PlaneMovementStats,
    config::GameConfig,
    events::PlayerDeath,
    gamestate::GameState,
    input::Intent,
    misc::{CollisionRadius, HP, VerticallyBounded},
    mods::{
        body::{BodyType, BomberBody, HeavyBody, MeleeBody, NormalBody, NukeBody},
        engines::{EngineType, GungineEngine, NormalEngine, SuperboostEngine},
        guns::*,
        // Recalculated,
    },
    physics::Physics,
    sprite::{AnimationIndices, TextureAtlasHashMap},
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
    texture_atlas_map: ResMut<TextureAtlasHashMap>,
) -> Result<(), BevyError> {
    let bullet_image_handle = asset_server.get_handle("images/bullet.png").unwrap();
    let mut root = commands.spawn((
        Visibility::Visible,
        Player,
        Intent::default(),
        HP {
            hp: 100.0,
            max: 100.0,
            regen: 20.0,
        },
        Physics {
            mass: 100.0,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            gravity: Vec3::new(0.0, -4.0, 0.0),
            friction: 0.99,
        },
        VerticallyBounded,
        PlayerStats::default(),
        PlaneMovementStats {
            acceleration: 10.0,
            turn_speed: 4.0,
        },
        CollisionRadius(10.0),
        Sprite {
            image: asset_server.get_handle("images/player.png").unwrap(),
            // texture_atlas: Some(TextureAtlas {
            //     layout: texture_atlas_map.get("player").unwrap().clone(),
            //     index: 0,
            // }),
            ..Default::default()
        },
        AnimationIndices { first: 0, last: 60 },
        Transform {
            scale: Vec3::splat(0.3),
            translation: Vec3::new(0.0, 0.0, 1.0), // put on Z layer 1, above the background.
            ..Default::default()
        },
    ));

    let mut commands = &mut root;

    commands = match userdata.selected_build.0 {
        WeaponType::MachineGun => {
            let bundle =
                WeaponType::MachineGun.data_from_type_and_handle(bullet_image_handle.clone());
            let WeaponSubtype::BulletBased {
                velocity,
                gravity,
                bullet_mass,
                friction,
                bullet_scale: _,
                num_spawned_per_shot,
            } = bundle.subtype
            else {
                panic!();
            };
            commands.insert(WeaponData {
                subtype: WeaponSubtype::BulletBased {
                    bullet_scale: 0.9,
                    velocity,
                    gravity,
                    bullet_mass,
                    friction,
                    num_spawned_per_shot,
                },
                ..bundle
            })
        }
        WeaponType::SlugGun => {
            let bundle = WeaponType::SlugGun.data_from_type_and_handle(bullet_image_handle.clone());
            let WeaponSubtype::BulletBased {
                velocity,
                gravity,
                bullet_mass,
                friction,
                bullet_scale: _,
                num_spawned_per_shot,
            } = bundle.subtype
            else {
                panic!();
            };
            commands.insert(WeaponData {
                subtype: WeaponSubtype::BulletBased {
                    bullet_scale: 0.9,
                    velocity,
                    gravity,
                    bullet_mass,
                    friction,
                    num_spawned_per_shot,
                },
                ..bundle
            })
        }
        WeaponType::Laser => commands
            .insert(WeaponType::Laser.data_from_type_and_handle(bullet_image_handle.clone())),
        WeaponType::Missile => todo!(),
        WeaponType::SpreadGun => {
            let bundle = WeaponType::SpreadGun.data_from_type_and_handle(bullet_image_handle.clone());
            let WeaponSubtype::BulletBased {
                velocity,
                gravity,
                bullet_mass,
                friction,
                bullet_scale: _,
                num_spawned_per_shot,
            } = bundle.subtype
            else {
                panic!();
            };
            commands.insert(WeaponData {
                subtype: WeaponSubtype::BulletBased {
                    bullet_scale: 0.9,
                    velocity,
                    gravity,
                    bullet_mass,
                    friction,
                    num_spawned_per_shot,
                },
                ..bundle
            })
        }
        WeaponType::Gungine => {
            panic!("gungine is not a selectable gun type");
        }
    };
    commands = match userdata.selected_build.1 {
        BodyType::Normal => commands.insert(NormalBody::default()),
        BodyType::Heavy => commands.insert(HeavyBody::default()),
        BodyType::Melee => commands.insert(MeleeBody::default()),
        BodyType::Nuke => commands.insert(NukeBody::default()),
        BodyType::Bomber => commands.insert(BomberBody::default()),
    };
    match userdata.selected_build.2 {
        EngineType::Normal => commands.insert(NormalEngine::default()),
        EngineType::Superboost => commands.insert(SuperboostEngine::new(
            game_config.superboost_acceleration_modifier,
            game_config.superboost_turn_speed_modifier,
        )),
        EngineType::Gungine => {
            commands.insert(GungineEngine::default());
            commands.with_children(|e| {
                e.spawn((
                    Transform::IDENTITY,
                    Visibility::Visible,
                    WeaponType::Gungine.data_from_type_and_handle(bullet_image_handle),
                ));
            })
        }
        EngineType::Submarine => todo!(),
    };
    Ok(())
}

pub fn plane_intent_movement_system(
    time: Res<Time>,
    mut query: Query<(&Intent, &PlaneMovementStats, &mut Physics, &mut Transform)>,
) {
    for (intent, stats, mut physics, mut transform) in query.iter_mut() {
        transform.rotate_z(intent.turn_intent * stats.turn_speed * time.delta_secs());

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
            event_writer.write(PlayerDeath);
        }
    }
}

pub fn player_death_system_stage_one(
    mut game_state: ResMut<NextState<GameState>>,
    mut events: EventReader<PlayerDeath>,
) {
    if !events.is_empty() {
        events.clear();

        // TODO: spawn vfx for death
        // TODO: queue player death sound playing, using another system and listening to PlayerDeath

        // set next game state
        game_state.set(GameState::GameEnding);
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
        commands.entity(query.single().unwrap().0).despawn();
    }
}
