use std::time::Duration;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_kira_audio::prelude::*;

mod ai;
mod body_type_stats;
mod camera;
mod config;
mod enemy;
mod events;
mod gamestate;
mod input;
mod loading;
mod misc;
mod mods;
mod physics;
mod player;
mod sfx;
mod sprite;
mod ui;
mod userdata;
mod vfx;

// use bevy_egui::EguiPlugin;
use mods::{
    guns::{GunCollectionPlugin, WeaponSubsystemPlugin},
    BodyModsPlugin,
};
use sfx::Sfx as SfxPlugin;
use vfx::VfxPlugin;

use camera::CameraPlugin;
use config::GameConfig;
use enemy::EnemyPlugin;
use events::EventsPlugin;
use gamestate::{game_ending_system, GameEndingTimer, GameState};
use input::player_intent_input_system;
use loading::{game_setup, load_assets, AssetsTracking};
use misc::{
    hp_regen_system, lifetime_postprocess_system, lifetime_system, score::ScorePlugin,
    vertical_bound_system, MiscPlugin,
};
use physics::linear_physics;
use player::{
    add_player, plane_intent_movement_system, player_death_detection_system,
    player_death_system_stage_one, player_death_system_stage_two,
};
// use sprite::CommonSprites;
use ui::{
    main_menu_ui_system, setup_main_menu_ui, /* HUDPlugin, */ MainMenuDebounceTimer,
    PausePlugin,
};
use userdata::UserData;

fn setup_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.get_handle("images/background.png"),
        transform: Transform {
            ..Default::default()
        },
        ..Default::default()
    }); // TODO: change this to a dynamic background that adapts to where the player is, such that an infinite scrolling effect can be achieved.
}

#[derive(Resource, DerefMut, Deref)]
pub struct DebugTimer(Timer);

fn debug_timer_ticker(time: Res<Time>, mut timer: ResMut<DebugTimer>) {
    timer.tick(time.delta());
}

fn main() {
    // add the following to restrict window size and set a title

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        // debug
        .insert_resource(DebugTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Repeating,
        ))) // debug timer
        .add_system(debug_timer_ticker)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // setup loading phase
        .add_state(GameState::Loading)
        .insert_resource(AssetsTracking::new())
        .insert_resource(UserData::new())
        .insert_resource(GameConfig::default())
        .insert_resource(CommonSprites::default())
        .insert_resource(GameEndingTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Once,
        )))
        .insert_resource(MainMenuDebounceTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Once,
        )))
        // insert system to handle userdata loading and saving
        .add_plugin(RonAssetPlugin::<GameConfig>::new(&["stats.ron"]))
        .add_system(load_assets.in_schedule(OnEnter(GameState::Loading)))
        .add_system(game_setup.in_set(OnUpdate(GameState::Loading)))
        .add_system(setup_main_menu_ui.in_schedule(OnEnter(GameState::MainMenu)))
        .add_system(main_menu_ui_system.in_set(OnUpdate(GameState::MainMenu)))
        // global event types
        .add_plugin(EventsPlugin)
        // setup and update for in-game
        .add_system((setup_sprites, add_player).in_schedule(OnEnter(GameState::InGame)))
        .add_system(
            (
                player_intent_input_system,
                plane_intent_movement_system,
                linear_physics,
                lifetime_system,
                vertical_bound_system,
                player_death_detection_system,
                player_death_system_stage_one,
                hp_regen_system,
            )
                .in_set(OnUpdate(GameState::InGame)),
        )
        .add_system(
            (game_ending_system, player_death_system_stage_two)
                .in_set(OnUpdate(GameState::GameEnding)),
        )
        .add_plugin(VfxPlugin)
        .add_plugin(SfxPlugin)
        .add_plugin(MiscPlugin)
        .add_plugin(ScorePlugin)
        // .add_plugin(HUDPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(BodyModsPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(GunCollectionPlugin)
        .add_plugin(WeaponSubsystemPlugin)
        .add_system(lifetime_postprocess_system.in_base_set(CoreSet::PostUpdate))
        .run();
}
