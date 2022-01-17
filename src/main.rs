use std::time::Duration;

use bevy::prelude::*;

mod bullet;
mod misc;
mod physics;
mod player;
mod sprite;

use misc::{lifetime_postprocess_system, lifetime_system, vertical_bound_system};
use physics::{linear_physics, Physics};
use player::{add_player, bullet_fire_system, player_input_system, Player};
use serde::{Deserialize, Serialize};
// use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    // GameOver,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    player_acceleration: f32,
    upper_bound: f32,
    upper_repulsion_strength: f32,
    lower_bound: f32,
    lower_repulsion_strength: f32,
}

// #[derive]
pub struct BulletFired {
    entity: Entity, // the entity that fired the bullet
    hostile: bool,
    // location: Vec3,
    // velocity: Vec3,
}

pub struct Game {
    pub config: Config,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform {
            // translation: Vec3::new(0.0, 20.0, 0.0),
            // scale: Vec3::splat(1.0 / 9.0),
            ..Default::default()
        },
        ..Default::default()
    }); // TODO: change this to a dynamic background that adapts to where the player is, such that an infinite scrolling effect can be achieved.
        // let config = asset_server.load("config.ron");
}

fn camera_system(
    _time: Res<Time>,
    _game: Res<Game>,
    mut cam_and_player: QuerySet<(
        QueryState<&mut Transform, With<Camera>>,
        QueryState<(&Transform, &Physics), With<Player>>,
    )>,
) {
    // keep camera focused on the player, with some influence from how they're moving and where they're aiming.
    let (player_translation, player_velocity, player_rotation) = {
        let (temp_transform, temp_physics) = cam_and_player.q1().single();
        (
            temp_transform.translation,
            temp_physics.velocity,
            temp_transform.rotation, //.angle_between(Quat::IDENTITY),
        )
    };

    let mut q0 = cam_and_player.q0();
    let mut transform = q0.single_mut();

    let velocity_len = player_velocity.length();

    transform.translation = player_translation
        + player_velocity.normalize() * velocity_len.clamp(0.0, 100.0) // push camera in velocity direction, clamped to some maximum value (to prevent the player from being off-screen)
        + player_rotation * Vec3::new(0.0, 1.0, 0.0) * 10.0; // push camera in aiming direction slightly.
}

fn debug_timer_ticker(time: Res<Time>, mut timer: ResMut<Timer>) {
    timer.tick(time.delta());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // debug
        .insert_resource(Timer::new(Duration::from_millis(500), true)) // debug timer
        .add_system(debug_timer_ticker)
        // gamestate
        .add_state(GameState::Playing)
        .insert_resource(Game {
            config: Config {
                player_acceleration: 12.0,
                upper_bound: 500.0,
                upper_repulsion_strength: 8.1,
                lower_bound: -500.0,
                lower_repulsion_strength: 16.1,
            },
        })
        // setup
        .add_startup_system(setup)
        .add_startup_system(add_player)
        // movement and physics
        .add_event::<BulletFired>()
        .add_system(player_input_system)
        .add_system(bullet_fire_system)
        .add_system(linear_physics)
        .add_system(lifetime_system)
        .add_system(vertical_bound_system)
        .add_system_to_stage(CoreStage::PostUpdate, lifetime_postprocess_system)
        // camera
        .add_system(camera_system)
        .run();
}
