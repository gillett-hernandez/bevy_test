use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Instant;

use bevy::asset::{LoadState, LoadedFolder};

use bevy::scene::ron::ser::Serializer;
use bevy::{asset::RecursiveDependencyLoadState, prelude::*, scene::ron::ser::PrettyConfig};

use serde::{Deserialize, Serialize};
// use bevy_kira_audio::AudioSource;

use crate::sprite::TextureAtlasHashMap;
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

#[derive(Resource, Deref, DerefMut)]
pub struct BakeTargets {
    pub paths: Vec<PathBuf>,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct BakeTargetsIntermediate {
    pub paths: Vec<(PathBuf, PathBuf, Handle<LoadedFolder>, Instant)>,
}

pub fn bake_assets(
    targets: Res<BakeTargets>,
    mut intermediate: ResMut<BakeTargetsIntermediate>,
    asset_server: ResMut<AssetServer>,
    folders: Res<Assets<LoadedFolder>>,
    mut images: ResMut<Assets<Image>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut loading: ResMut<AssetsTracking>,
    mut atlas_map: ResMut<TextureAtlasHashMap>,
) -> Result<(), BevyError> {
    if intermediate.len() == 0 {
        let mut targets = targets
            .iter()
            .filter_map(|e| {
                let target_base = PathBuf::from("assets")
                    .join("images")
                    .join("atlas")
                    .join(e.file_name().unwrap());
                let target_png = target_base.clone().with_extension("png");
                let target_ron = std::path::absolute(target_base.clone().with_extension("ron"))
                    .expect("failed to convert relative path to absolute path");

                let key = target_png
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                warn!("initiating folder load for {key}");

                (!(target_png.exists() && target_ron.exists())).then(|| {
                    (
                        target_png,
                        target_ron,
                        asset_server.load_folder(e.clone()),
                        Instant::now(),
                    )
                })
            })
            .collect::<Vec<_>>();

        targets
            .iter()
            .for_each(|(_, _, handle, _)| loading.add(handle.clone().untyped()));
        intermediate.extend(targets.drain(..));
    }

    for idx in (0..intermediate.len()).rev() {
        warn!(
            "state for asset {:?} = {:?}",
            intermediate[idx].0,
            asset_server.get_load_states(&intermediate[idx].2)
        );
        if matches!(
            asset_server.get_load_state(&intermediate[idx].2),
            Some(LoadState::Loaded)
        ) {
            // all assets under folder loaded
            warn!("starting builder");
            let mut binding = TextureAtlasBuilder::default();
            let builder = binding.auto_format_conversion(true);

            let (target_png, target_ron, folder_handle, start_time) = intermediate.swap_remove(idx);

            let key = target_png
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            warn!("building atlas map for {key}");

            let folder = folders.get(&folder_handle).unwrap();
            let folder_images = folder
                .handles
                .iter()
                .flat_map(|e| e.clone().try_typed::<Image>())
                .flat_map(|e| images.get(&e))
                .collect::<Vec<_>>();

            for image in folder_images {
                builder.add_texture(None, image);
            }

            let (layout, sources, actual_atlas) = builder.build()?;

            // TODO: add serialization and deserialization for Atlases. Needs research into how to serialize a Image, as the existing mechanisms seem clunky
            // warn!("writing layout file to {:?}", target_ron);
            // let layout_file = File::create(target_ron)?;
            // let mut serializer = Serializer::new(layout_file, None)?;
            // layout.serialize(&mut serializer)?;

            let layout_handle = atlas_layouts.add(layout);

            let atlas_handle = images.add(actual_atlas);

            warn!(
                "building and loading finished for atlas {} in time {}",
                key,
                start_time.elapsed().as_millis() as f32 / 1000.0
            );
            atlas_map.insert(key, (atlas_handle, layout_handle));
        }
    }
    Ok(())
}

const GAME_CONFIG_FILE: &'static str = "config.ron";
const USER_CONFIG_FILE: &'static str = "userdata.ron";

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsTracking>,
) -> Result<(), BevyError> {
    // static sprites
    for image_path in [
        "images/background.png",
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
    Ok(())
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
    mut user_data: ResMut<UserData>,
    mut state: ResMut<NextState<GameState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsTracking>,
    game_config_asset: Res<Assets<GameConfig>>,
    user_data_asset: Res<Assets<UserData>>,
    targets: Res<BakeTargetsIntermediate>,
    // atlas_map: Res<TextureAtlasHashMap>,
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

    if targets.len() > 0 {
        all_done = false;
    }

    if all_done {
        *game_config = game_config_asset
            .get(server.get_handle(GAME_CONFIG_FILE).unwrap().id())
            .unwrap()
            .clone();

        *user_data = user_data_asset
            .get(server.get_handle(USER_CONFIG_FILE).unwrap().id())
            .unwrap()
            .clone();

        state.set(GameState::MainMenu);
    }
}
