use std::path::Path;

use bevy::scene::ron::ser::Serializer;
use bevy::{asset::RecursiveDependencyLoadState, prelude::*, scene::ron::ser::PrettyConfig};
use serde::Serialize;
// use bevy_kira_audio::AudioSource;

use crate::{config::GameConfig, gamestate::GameState, userdata::UserData};

#[derive(Resource, Deref)]
pub struct AssetsTracking(pub Vec<UntypedHandle>);
impl AssetsTracking {
    pub fn new() -> Self {
        AssetsTracking(vec![])
    }
    pub fn add(&mut self, handle: UntypedHandle) {
        self.0.push(handle);
    }
}

const GAME_CONFIG_FILE: &'static str = "config.ron";
const USER_CONFIG_FILE: &'static str = "userdata.ron";

pub fn load_assets(
    // mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsTracking>,
) {
    // pngs
    for image_path in [
        "images/background.png",
        "images/player.png",
        "images/bullet.png",
        "images/enemy/basic_enemy.png",
    ] {
        let handle: Handle<Image> = asset_server.load(image_path);
        loading.add(handle.untyped());
    }

    for audio_path in ["sfx/hit_sound.ogg"] {
        let handle: Handle<AudioSource> = asset_server.load(audio_path);
        loading.add(handle.clone().untyped());
    }
    // stats
    let path = Path::new("assets").join(USER_CONFIG_FILE);
    if let Ok(file) = std::fs::File::create_new(path) {
        // will error if the file already exists
        let mut serializer = Serializer::new(file, Some(PrettyConfig::new().depth_limit(4)))
            .expect("couldn't create serializer");
        let result = UserData::default().serialize(&mut serializer);
        result.expect("could not write to file");
    }

    let path = Path::new("assets").join(GAME_CONFIG_FILE);
    if let Ok(file) = std::fs::File::create_new(path) {
        // will error if the file already exists
        let mut serializer = Serializer::new(file, Some(PrettyConfig::new().depth_limit(4)))
            .expect("couldn't create serializer");
        let result = GameConfig::default().serialize(&mut serializer);
        result.expect("could not write to file");
    }

    let handle: Handle<UserData> = asset_server.load(USER_CONFIG_FILE);
    loading.add(handle.untyped());
    let handle: Handle<GameConfig> = asset_server.load(GAME_CONFIG_FILE);
    loading.add(handle.untyped());

    info!("loading {} items", loading.0.len());
}

pub fn loading_state_watcher<T: Asset>(mut loads: EventReader<AssetEvent<T>>) {
    for load in loads.read() {
        match load {
            AssetEvent::Added { id } => {
                info!("asset {} added", id.to_string());
            }
            AssetEvent::Modified { id } => {
                info!("asset {} modified", id.to_string());
            }
            AssetEvent::Removed { id } => {
                info!("asset {} removed", id.to_string());
            }
            AssetEvent::LoadedWithDependencies { id } => {
                info!("asset {} loaded with deps", id.to_string());
            }
            AssetEvent::Unused { id } => {
                warn!("asset {} is unsued", id.to_string());
            }
        }
    }
}

pub fn loading_update(
    mut game_config: ResMut<GameConfig>,
    mut state: ResMut<NextState<GameState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsTracking>,
    game_config_asset: Res<Assets<GameConfig>>,
) {
    // splash screen, loading progress, and transition to main menu

    // TODO: splash screen

    let mut all_done = true;
    for handle in loading.iter() {
        match server.get_load_states(handle.id()).map(|tuple| tuple.2) {
            Some(RecursiveDependencyLoadState::Loaded) => {}
            Some(RecursiveDependencyLoadState::Failed(e)) => {
                error!(
                    "asset {} failed to load with error {}",
                    handle.id().to_string(),
                    e.to_string()
                );
            }
            _ => {
                all_done = false;
            }
        }
    }
    if all_done {
        *game_config = game_config_asset
            .get(server.get_handle(GAME_CONFIG_FILE).unwrap().id())
            .unwrap()
            .clone();

        state.set(GameState::MainMenu);
    }
}
