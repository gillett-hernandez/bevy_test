use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::{
    config::GameConfig,
    gamestate::GameState,
    // sprite::{CommonSprites, HPCircleSprite},
};

#[derive(Resource)]
pub struct AssetsTracking(Vec<HandleUntyped>);
impl AssetsTracking {
    pub fn new() -> Self {
        AssetsTracking(vec![])
    }
    pub fn add(&mut self, handle: HandleUntyped) {
        self.0.push(handle);
    }
}

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
        loading.add(handle.clone_untyped());
    }

    for audio_path in ["sfx/hit_sound.ogg"] {
        let handle: Handle<AudioSource> = asset_server.load(audio_path);
        loading.add(handle.clone_untyped());
    }
    // stats
    let handle: Handle<GameConfig> = asset_server.load("data.stats.ron");
    loading.add(handle.clone_untyped());
}

pub fn game_setup(
    // mut common_sprites: ResMut<CommonSprites>,
    mut game_config: ResMut<GameConfig>,
    mut state: ResMut<NextState<GameState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsTracking>,
    game_config_asset: Res<Assets<GameConfig>>,

    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // splash screen, loading progress, and transition to main menu
    use bevy::asset::LoadState;

    // TODO: splash screen

    match server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            // one of our assets had an error
            panic!("asset failed to load");
        }
        LoadState::Loaded => {
            // all assets are now ready

            *game_config = game_config_asset
                .get(&server.get_handle("data.stats.ron"))
                .unwrap()
                .clone();

            // don't remove the resource to keep the resources loaded
            state.set(GameState::MainMenu);
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}
