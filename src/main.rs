use std::time::Duration;

use ai::basic::plane_ai;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    time::*,
};

mod ai;
mod body_type_stats;
mod camera;
mod config;
mod enemy;
mod events;
mod gamestate;
mod loading;
mod misc;
mod mods;
mod pause;
mod physics;
mod player;
mod sprite;
mod ui;
mod userdata;

// use bevy_egui::EguiPlugin;
use mods::guns::{BulletCollisionPlugin, GunCollectionPlugin, LaserCollisionPlugin};

use camera::CameraPlugin;
use config::Config;
use enemy::EnemyPlugin;
use events::EventsPlugin;
use gamestate::{Game, GameState};
use loading::{game_setup, load_assets, AssetsTracking};
use misc::{hp_regen_system, lifetime_postprocess_system, lifetime_system, vertical_bound_system};
use physics::linear_physics;
use player::{
    add_player, player_death_detection_system, player_movement_input_system,
    player_movement_physics_system,
};
use ui::{main_menu_ui_system, setup_main_menu_ui};
use userdata::UserData;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.get_handle("background.png"),
        transform: Transform {
            ..Default::default()
        },
        ..Default::default()
    }); // TODO: change this to a dynamic background that adapts to where the player is, such that an infinite scrolling effect can be achieved.
        // let config = asset_server.load("config.ron");
}

#[derive(Resource)]
pub struct DebugTimer(Timer);

fn debug_timer_ticker(time: Res<Time>, mut timer: ResMut<DebugTimer>) {
    timer.0.tick(time.delta());
}

fn main() {
    // add the following to restrict window size and set a title

    App::new()
        .add_plugins(DefaultPlugins)
        // debug
        .insert_resource(DebugTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Repeating,
        ))) // debug timer
        .add_system(debug_timer_ticker)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // setup loading phase
        .add_state(GameState::Loading)
        .insert_resource(AssetsTracking::new())
        .insert_resource(UserData::new())
        .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_assets))
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(game_setup))
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu_ui))
        .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(main_menu_ui_system))
        // TODO: change this config to load from a file.
        .insert_resource(Game {
            config: Config {
                vertical_bounds_rotation_speed: 3.0, // radians/sec
                upper_bound: 500.0,
                upper_repulsion_strength: 8.1, // pixels/sec^2
                lower_bound: -500.0,
                lower_repulsion_strength: 16.1, // pixels/sec^2
            },
        })
        // global event types
        .add_plugin(EventsPlugin)
        // setup and update for in-game
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(setup)
                .with_system(add_player),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_movement_input_system)
                .with_system(player_movement_physics_system)
                .with_system(linear_physics)
                .with_system(lifetime_system)
                .with_system(vertical_bound_system)
                .with_system(player_death_detection_system)
                .with_system(hp_regen_system),
        )
        .add_plugin(EnemyPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(GunCollectionPlugin)
        .add_plugin(BulletCollisionPlugin)
        .add_system_to_stage(CoreStage::PostUpdate, lifetime_postprocess_system)
        // camera
        .run();
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(EguiPlugin)
    //     .run();
}
