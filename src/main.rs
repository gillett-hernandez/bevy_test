use std::time::Duration;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

mod ai;
mod bullet;
mod camera;
mod config;
mod enemy;
mod events;
mod gamestate;
mod gun_collection;
mod loading;
mod misc;
mod pause;
mod physics;
mod player;
mod sprite;

use camera::CameraPlugin;
use config::Config;
use events::BulletFired;
use gamestate::{Game, GameState};
use gun_collection::{GunCollectionPlugin, MachineGun, SlugGun};
use loading::{load_assets, watch_loading_progress, AssetsTracking};
use misc::{lifetime_postprocess_system, lifetime_system, vertical_bound_system};
use physics::linear_physics;
use player::{add_player, player_movement_input_system};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.get_handle("background.png"),
        transform: Transform {
            // translation: Vec3::new(0.0, 20.0, 0.0),
            // scale: Vec3::splat(1.0 / 9.0),
            ..Default::default()
        },
        ..Default::default()
    }); // TODO: change this to a dynamic background that adapts to where the player is, such that an infinite scrolling effect can be achieved.
        // let config = asset_server.load("config.ron");
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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // setup loading phase
        .add_state(GameState::Loading)
        .insert_resource(AssetsTracking::new())
        .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_assets))
        .add_system_set(
            SystemSet::on_update(GameState::Loading).with_system(watch_loading_progress),
        )
        // TODO: change this config to load from a file.
        .insert_resource(Game {
            config: Config {
                player_acceleration: 12.0,           // pixels/sec^2
                player_rotation_speed: 3.1,          // radians/sec
                vertical_bounds_rotation_speed: 3.0, // radians/sec
                upper_bound: 500.0,
                upper_repulsion_strength: 8.1, // pixels/sec^2
                lower_bound: -500.0,
                lower_repulsion_strength: 16.1, // pixels/sec^2
            },
        })
        // global event types
        .add_event::<BulletFired<MachineGun>>()
        .add_event::<BulletFired<SlugGun>>()
        // setup and update for in-game
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(setup)
                .with_system(add_player),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_movement_input_system)
                .with_system(linear_physics)
                .with_system(lifetime_system)
                .with_system(vertical_bound_system),
        )
        .add_plugin(CameraPlugin)
        .add_plugin(GunCollectionPlugin {})
        .add_system_to_stage(CoreStage::PostUpdate, lifetime_postprocess_system)
        // camera
        .run();
}
