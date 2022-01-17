use std::{marker::PhantomData, time::Duration};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

mod bullet;
mod gun_collection;
mod misc;
mod physics;
mod player;
mod sprite;

use gun_collection::{GunCollectionPlugin, MachineGun, SlugGun};
use misc::{lifetime_postprocess_system, lifetime_system, vertical_bound_system};
use physics::{linear_physics, Physics};
use player::{add_player, player_movement_input_system, Player};
use serde::{Deserialize, Serialize};
// use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    InGame,
    Paused,
    // GameOver,
}

struct AssetsTracking(Vec<HandleUntyped>);
impl AssetsTracking {
    pub fn new() -> Self {
        AssetsTracking(vec![])
    }
    pub fn add(&mut self, handle: HandleUntyped) {
        self.0.push(handle);
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    player_acceleration: f32,
    player_rotation_speed: f32,
    vertical_bounds_rotation_speed: f32,
    upper_bound: f32,
    upper_repulsion_strength: f32,
    lower_bound: f32,
    lower_repulsion_strength: f32,
}

// #[derive]
pub struct BulletFired<T> {
    // where T is the type of bullet fired
    entity: Entity, // the entity that fired the bullet
    hostile: bool,
    phantom: PhantomData<*const T>,
    // location: Vec3,
    // velocity: Vec3,
}

impl<T> BulletFired<T> {
    pub fn new(entity: Entity, hostile: bool) -> Self {
        BulletFired {
            entity,
            hostile,
            phantom: PhantomData,
        }
    }
}

unsafe impl<T> Send for BulletFired<T> {}
unsafe impl<T> Sync for BulletFired<T> {}

pub struct Game {
    pub config: Config,
}

struct PauseTimer(Timer);

fn load_assets(
    // mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsTracking>,
) {
    let paths = vec!["background.png", "player.png"];
    for path in paths {
        let handle: Handle<Image> = asset_server.load(path);
        loading.add(handle.clone_untyped());
    }
}

fn watch_loading_progress(
    // mut commands: Commands,
    mut state: ResMut<State<GameState>>,

    server: Res<AssetServer>,
    loading: Res<AssetsTracking>,
) {
    use bevy::asset::LoadState;

    match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {
            // one of our assets had an error
            panic!("asset failed to load");
        }
        LoadState::Loaded => {
            // all assets are now ready

            // don't remove the resource to keep the resources loaded
            // commands.remove_resource::<AssetsLoading>();
            state.set(GameState::InGame).unwrap();
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}

fn pause_menu_system(
    _keyboard_input: Res<Input<KeyCode>>,
    _time: Res<Time>,
    _pause_timer: ResMut<PauseTimer>,
    _game_state: ResMut<State<GameState>>,
) {
    // TODO: implement pausing such that the camera switches to the UI camera and the player doesn't get recreated and stuff.
    // if pause_timer.0.tick(time.delta()).finished() {
    //     if keyboard_input.just_pressed(KeyCode::Escape) {
    //         game_state.set(GameState::InGame).unwrap();
    //     }
    //     pause_timer.0.reset();
    // }
}

fn pause_input_handler(
    _keyboard_input: Res<Input<KeyCode>>,
    _time: Res<Time>,
    _pause_timer: ResMut<PauseTimer>,
    _game_state: ResMut<State<GameState>>,
) {
    // TODO: implement pausing such that the camera switches to the UI camera and the player doesn't get recreated and stuff.
    // if pause_timer.0.tick(time.delta()).finished() {
    //     if keyboard_input.just_pressed(KeyCode::Escape) {
    //         game_state.set(GameState::Paused).unwrap();
    //     }
    //     pause_timer.0.reset();
    // }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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
        + player_velocity.normalize() * 100.0 * (1.0 - (-velocity_len/100.0).exp()) // push camera in velocity direction, clamped to some maximum value (to prevent the player from being off-screen)
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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // setup loading phase
        .add_state(GameState::Loading)
        .insert_resource(AssetsTracking::new())
        .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_assets))
        .add_system_set(
            SystemSet::on_update(GameState::Loading).with_system(watch_loading_progress),
        )
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
        .insert_resource(PauseTimer(Timer::new(Duration::from_millis(200), false)))
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
                .with_system(vertical_bound_system)
                .with_system(camera_system)
                .with_system(pause_input_handler),
        )
        .add_system_set(SystemSet::on_update(GameState::Paused).with_system(pause_menu_system))
        .add_plugin(GunCollectionPlugin {})
        .add_system_to_stage(CoreStage::PostUpdate, lifetime_postprocess_system)
        // camera
        .run();
}
