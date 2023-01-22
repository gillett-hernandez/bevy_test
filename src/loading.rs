use bevy::prelude::*;

use crate::{config::GameConfig, gamestate::GameState};

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
    let paths = vec![
        "background.png",
        "player.png",
        "bullet.png",
        "enemy/basic_enemy.png",
    ];
    for path in paths {
        let handle: Handle<Image> = asset_server.load(path);
        loading.add(handle.clone_untyped());
    }
    let handle: Handle<GameConfig> = asset_server.load("data.stats.ron");
    loading.add(handle.clone_untyped());
}

pub fn game_setup(
    // mut commands: Commands,
    mut game_config: ResMut<GameConfig>,
    mut state: ResMut<State<GameState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsTracking>,
    game_config_asset: Res<Assets<GameConfig>>,
) {
    // splash screen, loading progress, and transition to main menu
    use bevy::asset::LoadState;

    // TODO: splash screen

    match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
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
            // commands.remove_resource::<AssetsLoading>();
            state.set(GameState::MainMenu).unwrap();
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}
