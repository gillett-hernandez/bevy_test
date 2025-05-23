use std::{path::Path, time::Duration};

use bevy::{
    audio::{AudioPlugin, SpatialScale},
    log::LogPlugin,
    prelude::*,
};
use bevy_common_assets::ron::RonAssetPlugin;
// use bevy_kira_audio::prelude::*;

mod ai;
mod body_type_stats;
mod camera;
mod config;
mod enemy;
mod events;
mod gamestate;
mod input;
mod loading;
mod log;
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
use camera::CameraPlugin;
use config::GameConfig;
use enemy::EnemyPlugin;
use events::EventsPlugin;
use gamestate::{GameEndingTimer, GameState, game_ending_system};
use input::player_input_intent_system;
use loading::{
    AssetsTracking, BakeTargets, BakeTargetsIntermediate, bake_assets, load_assets, loading_update,
};
use log::CustomLogPlugin;
use misc::{
    MiscPlugin,
    hitstun::{HitStun, in_game_no_hitstun},
    hp_regen_system, lifetime_postprocess_system, lifetime_system,
    score::ScorePlugin,
    vertical_bound_system,
};
use mods::{
    BodyModsPlugin,
    guns::{GunCollectionPlugin, WeaponSubsystemPlugin},
};
use physics::linear_physics;
use player::{
    add_player, animate_player_sprite, plane_intent_movement_system, player_death_detection_system,
    player_death_system_stage_one, player_death_system_stage_two,
};
use sfx::Sfx as SfxPlugin;
use sprite::TextureAtlasHashMap;
use vfx::{VfxPlugin, hp::hp_effect_setup_system};

use userdata::UserData;

use crate::{loading::loading_state_watcher, ui::GameUIPlugin};

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite {
        image: asset_server.get_handle("images/background.png").unwrap(),

        ..Default::default()
    }); // TODO: change this to a dynamic background that adapts to where the player is, such that an infinite scrolling effect can be achieved.
}

#[derive(Resource, DerefMut, Deref)]
pub struct DebugTimer(Timer);

fn debug_timer_ticker(time: Res<Time>, mut timer: ResMut<DebugTimer>) {
    timer.tick(time.delta());
}

fn observe_game_state(state: Res<State<GameState>>, debug_timer: Res<DebugTimer>) {
    if debug_timer.just_finished() {
        dbg!(state.get());
    }
}

fn main() {
    const AUDIO_SCALE: f32 = 1.0 / 100.0;
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .set(AudioPlugin {
                    default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
                    ..default()
                })
                .disable::<LogPlugin>(),
        )
        .add_plugins(CustomLogPlugin)
        // debug
        .insert_resource(DebugTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Repeating,
        ))) // debug timer
        .add_systems(Update, debug_timer_ticker)
        // setup loading phase
        .insert_state::<GameState>(GameState::Loading)
        .add_systems(Update, observe_game_state)
        .insert_resource(AssetsTracking::new())
        .insert_resource(TextureAtlasHashMap::default())
        .insert_resource(BakeTargets {
            paths: vec![Path::new("images").join("pre_atlas").join("player")],
        })
        .insert_resource(BakeTargetsIntermediate::default())
        .insert_resource(HitStun(false))
        .insert_resource(UserData::default())
        .insert_resource(GameConfig::default())
        .insert_resource(GameEndingTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Once,
        )))
        // insert system to handle userdata loading and saving
        .add_plugins(RonAssetPlugin::<UserData>::new(&["userdata.ron"]))
        .add_plugins(RonAssetPlugin::<GameConfig>::new(&["config.ron"]))
        .add_plugins((
            EventsPlugin,
            VfxPlugin,
            SfxPlugin,
            MiscPlugin,
            ScorePlugin,
            BodyModsPlugin,
            EnemyPlugin,
            CameraPlugin,
            GunCollectionPlugin,
            WeaponSubsystemPlugin,
            GameUIPlugin, // depends on PausePlugin, automatically adds it
                          // LogDiagnosticsPlugin::default(),
                          // FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            Update,
            (
                loading_update,
                bake_assets,
                loading_state_watcher::<Image>,
                loading_state_watcher::<GameConfig>,
                loading_state_watcher::<UserData>,
                loading_state_watcher::<AudioSource>,
            )
                .run_if(in_state(GameState::Loading)),
        )
        .add_systems(
            OnTransition {
                exited: GameState::MainMenu,
                entered: GameState::InGame,
            },
            (setup_background, add_player),
        )
        // // setup and update for in-game
        .add_systems(
            Update,
            (
                player_input_intent_system,
                animate_player_sprite,
                plane_intent_movement_system,
                linear_physics,
            )
                .chain()
                .run_if(in_game_no_hitstun),
        )
        .add_systems(
            Update,
            (
                lifetime_system,
                vertical_bound_system,
                player_death_detection_system,
                player_death_system_stage_one,
                hp_regen_system,
                hp_effect_setup_system,
            )
                .run_if(in_game_no_hitstun),
        )
        .add_systems(
            Update,
            (game_ending_system, player_death_system_stage_two)
                .run_if(in_state(GameState::GameEnding)),
        )
        .add_systems(PostUpdate, lifetime_postprocess_system)
        .run();
}
