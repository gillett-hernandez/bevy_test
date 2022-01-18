use bevy::prelude::*;

use crate::{
    events::PlayerDeath,
    gamestate::Game,
    gun_collection::*,
    misc::VerticallyBounded,
    physics::{Physics, Position},
};

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
        .insert(Position(Vec2::ZERO))
        .insert(VerticallyBounded {})
        .insert(GunType::MachineGun.data_from_type(asset_server.get_handle("bullet.png")))
        // .insert(Timers::new().with_pair(
        //     PlayerTimers::ShootTimer,
        //     Timer::new(Duration::from_millis(250), true),
        // ))
        .with_children(|e| {
            // add sprite as child so that it's affected by the transform of the parent
            e.spawn_bundle(SpriteBundle {
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

pub fn player_movement_input_system(
    // mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game: Res<Game>,
    time: Res<Time>,
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

        transform.rotation = transform.rotation
            * Quat::from_rotation_z(-game.config.player_rotation_speed * time.delta_seconds());
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        println!("KeyCode::Left pressed");
    }
    if keyboard_input.pressed(KeyCode::Left) {
        // turn left

        transform.rotation = transform.rotation
            * Quat::from_rotation_z(game.config.player_rotation_speed * time.delta_seconds());
    }

    // let shoot_timer = timers.timers.get_mut(&PlayerTimers::ShootTimer).unwrap();
}

pub fn player_hp_system(
    mut event_writer: EventWriter<PlayerDeath>,
    query: Query<(Entity, &Player)>,
) {
    for (entity, player) in query.iter() {
        if player.hp <= 0.0 {
            // kill player if hp drops <= 0
            // commands.entity(entity).despawn_recursive();
            event_writer.send(PlayerDeath)
        }
    }
}
