use bevy::prelude::*;

use crate::{gun_collection::*, misc::VerticallyBounded, physics::Physics, Game};

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum PlayerTimers {
//     ShootTimer,
// }

#[derive(Component)]
pub struct Player {
    pub hp: f32,
}

impl Player {
    pub fn new() -> Self {
        Player { hp: 100.0 }
    }
}

pub fn add_player(mut commands: Commands, mut _game: ResMut<Game>, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle((GlobalTransform::identity(), Transform::default()))
        .insert(Player::new())
        .insert(Physics {
            velocity: Vec3::new(0.0, 0.0, 0.0),
            gravity: Vec3::new(0.0, -4.0, 0.0),
            friction: 0.99,
        })
        .insert(VerticallyBounded {})
        .insert(SlugGun::new(asset_server.get_handle("player.png")))
        // .insert(Timers::new().with_pair(
        //     PlayerTimers::ShootTimer,
        //     Timer::new(Duration::from_millis(250), true),
        // ))
        .with_children(|e| {
            // add sprite as child so that it's affected by the transform of the parent
            e.spawn_bundle(SpriteBundle {
                texture: asset_server.load("player.png"),
                ..Default::default()
            });
        });
}

pub fn player_movement_input_system(
    // mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game: ResMut<Game>,
    // time: ResMut<Time>,
    mut query: Query<(Entity, &mut Physics, &mut Transform), With<Player>>,
    // config: Res<Assets<Config>>,
) {
    let (_entity, mut physics, mut transform) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Up) {
        println!("KeyCode::Up pressed, velocity = {}", physics.velocity);
    }
    if keyboard_input.pressed(KeyCode::Up) {
        // accelerate
        physics.velocity +=
            transform.rotation * Vec3::new(0.0, game.config.player_acceleration, 0.0);
        //  Vec3::splat(1.0);
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        println!("KeyCode::Down pressed");
    }
    if keyboard_input.pressed(KeyCode::Down) {
        // decelerate
        physics.velocity *= 0.99;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        println!("KeyCode::Right pressed");
    }
    if keyboard_input.pressed(KeyCode::Right) {
        // turn right

        transform.rotation = transform.rotation * Quat::from_rotation_z(-0.1);
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        println!("KeyCode::Left pressed");
    }
    if keyboard_input.pressed(KeyCode::Left) {
        // turn left

        transform.rotation = transform.rotation * Quat::from_rotation_z(0.1);
    }

    // let shoot_timer = timers.timers.get_mut(&PlayerTimers::ShootTimer).unwrap();
}
