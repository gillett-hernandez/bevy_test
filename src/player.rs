use bevy::prelude::*;

use crate::{
    body_type_stats::PlaneMovementStats,
    events::PlayerDeath,
    gamestate::Game,
    misc::{VerticallyBounded, HP},
    mods::{engines::NormalEngine, guns::*},
    physics::Physics,
};

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum PlayerTimers {
//     ShootTimer,
// }

#[derive(Component)]
pub struct Player;

#[derive(Component)]
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

pub fn add_player(mut commands: Commands, mut _game: ResMut<Game>, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpatialBundle::default())
        .insert(Player)
        .insert(HP {
            hp: 100.0,
            max: 100.0,
            regen: 10.0,
        })
        .insert(Physics {
            velocity: Vec3::new(0.0, 0.0, 0.0),
            gravity: Vec3::new(0.0, -4.0, 0.0),
            friction: 0.99,
        })
        .insert(VerticallyBounded)
        .insert(GunType::SlugGun.data_from_type(asset_server.get_handle("bullet.png")))
        .insert(PlayerStats::default())
        .insert(PlaneMovementStats {
            acceleration: 15.0,
            turn_speed: 3.0,
        })
        .insert(NormalEngine::default())
        // .insert(Engine::Normal)
        // .insert(Engine::Normal)
        // .insert(Timers::new().with_pair(
        //     PlayerTimers::ShootTimer,
        //     Timer::new(Duration::from_millis(250), true),
        // ))
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
}

#[derive(Component)]
pub struct Intent {
    // distilled input
    pub accelerate: bool,
    pub brake: bool,
    pub turn_intent: f32,
    pub fire: bool,
}

pub fn player_movement_input_system(
    // mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    // game: Res<Game>,
    // time: Res<Time>,
    // mut query: Query<(Entity, &mut Physics, &mut Transform), With<Player>>,
    mut query: Query<(Entity, &mut Intent), With<Player>>,
    // config: Res<Assets<Config>>,
) {
    let (_entity, mut intent) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Up) {
        // println!("KeyCode::Up pressed, velocity = {}", physics.velocity);
    }
    if keyboard_input.pressed(KeyCode::Up) {
        // accelerate
        // physics.velocity +=
        //     transform.rotation * Vec3::new(0.0, game.config.player_acceleration, 0.0);
        intent.accelerate = true;
        //  Vec3::splat(1.0);
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        // println!("KeyCode::Down pressed");
    }
    if keyboard_input.pressed(KeyCode::Down) {
        // decelerate
        intent.brake = true;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        // println!("KeyCode::Right pressed");
    }
    if keyboard_input.pressed(KeyCode::Right) {
        // turn right

        // transform.rotation *=
        //     Quat::from_rotation_z(-game.config.player_rotation_speed * time.delta_seconds());
        intent.turn_intent = 1.0;
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        // println!("KeyCode::Left pressed");
    }
    if keyboard_input.pressed(KeyCode::Left) {
        // turn left

        intent.turn_intent = -1.0;
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

pub fn player_movement_physics_system(
    time: Res<Time>,
    mut query: Query<(&Intent, &PlaneMovementStats, &mut Physics, &mut Transform), With<Player>>,
) {
    for (intent, stats, mut physics, mut transform) in query.iter_mut() {
        transform.rotate_z(intent.turn_intent * stats.turn_speed * time.delta_seconds());

        physics.velocity += stats.acceleration * (transform.rotation * Vec3::Y);
    }
}
