use std::time::Duration;

use bevy::prelude::*;
use misc::{lifetime_postprocess_system, lifetime_system};
use physics::linear_physics;
use player::{add_player, bullet_fire_system, move_player, Player};
use serde::{Deserialize, Serialize};
// use bevy_inspector_egui::WorldInspectorPlugin;

mod bullet;
mod misc;
mod physics;
mod player;
mod sprite;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    player_acceleration: f32,
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

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // commands.insert_resource();
    // commands.add_resource();
    // let config = asset_server.load("config.ron");
}

fn camera_system(
    _time: Res<Time>,
    _game: Res<Game>,
    mut cam_and_player: QuerySet<(
        QueryState<&mut Transform, With<Camera>>,
        QueryState<&Transform, With<Player>>,
    )>,
) {
    // keep camera focused on the player, with some influence from how they're moving and where they're aiming.
    let player_translation = cam_and_player.q1().single().translation;

    let mut q0 = cam_and_player.q0();
    let mut transform = q0.single_mut();

    transform.translation = player_translation;
}

fn debug_timer_ticker(time: Res<Time>, mut timer: ResMut<Timer>) {
    timer.tick(time.delta());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // debug
        .insert_resource(Timer::new(Duration::from_millis(500), false)) // debug timer
        .add_system(debug_timer_ticker)
        // gamestate
        .add_state(GameState::Playing)
        .insert_resource(Game {
            config: Config {
                player_acceleration: 70.0,
            },
        })
        // setup
        .add_startup_system(setup)
        .add_startup_system(add_player)
        // movement and physics
        .add_event::<BulletFired>()
        .add_system(move_player)
        .add_system(bullet_fire_system)
        .add_system(linear_physics)
        .add_system(lifetime_system)
        .add_system_to_stage(CoreStage::PostUpdate, lifetime_postprocess_system)
        // camera
        .add_system(camera_system)
        .run();
}
