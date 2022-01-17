use std::time::Duration;

use crate::{
    bullet::Bullet,
    misc::{Lifetime, Timers, VerticallyBounded},
    physics::Physics,
    BulletFired, Game,
};
use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerTimers {
    ShootTimer,
}

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
        .insert(Timers::new().with_pair(
            PlayerTimers::ShootTimer,
            Timer::new(Duration::from_millis(250), true),
        ))
        .with_children(|e| {
            // add sprite as child so that it's affected by the transform of the parent
            e.spawn_bundle(SpriteBundle {
                texture: asset_server.load("player.png"),
                ..Default::default()
            });
        });
}

pub fn player_input_system(
    // mut commands: Commands,
    mut event_writer: EventWriter<BulletFired>,
    keyboard_input: Res<Input<KeyCode>>,
    game: ResMut<Game>,
    time: ResMut<Time>,
    mut query: Query<
        (
            Entity,
            &mut Physics,
            &mut Transform,
            &mut Timers<PlayerTimers>,
        ),
        With<Player>,
    >,
    // config: Res<Assets<Config>>,
) {
    let (entity, mut physics, mut transform, mut timers) = query.single_mut();

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

    let shoot_timer = timers.timers.get_mut(&PlayerTimers::ShootTimer).unwrap();
    if keyboard_input.pressed(KeyCode::Space) && shoot_timer.tick(time.delta()).just_finished() {
        // fire bullet
        event_writer.send(BulletFired {
            entity,
            hostile: false,
        });
        shoot_timer.reset();
    }
}

pub fn bullet_fire_system(
    mut commands: Commands,
    mut event_reader: EventReader<BulletFired>,
    query: Query<(Entity, &Physics, &Transform)>,
    asset_server: Res<AssetServer>,
) {
    // for (e, velocity, transform, global_transform) in query.iter() {

    // }
    for event in event_reader.iter() {
        let (_e, physics, transform) = query.get(event.entity).unwrap();
        commands
            .spawn_bundle((
                GlobalTransform::identity(),
                transform.clone(),
                Bullet {
                    hostile: event.hostile,
                },
                Lifetime::new(Duration::from_millis(600)), // TODO: remove magic numbers by reading from config or from player data somehow.
                Physics {
                    velocity: physics.velocity + transform.rotation * Vec3::new(0.0, 1000.0, 0.0),
                    gravity: Vec3::ZERO,
                    friction: 0.99,
                },
            ))
            .with_children(|child_builder| {
                child_builder.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("player.png"),
                    transform: Transform {
                        scale: Vec3::splat(0.3),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    }
}
