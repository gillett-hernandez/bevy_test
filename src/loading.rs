use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

use bevy::asset::{DependencyLoadState, LoadedFolder};
use bevy::ecs::system::SystemState;
use bevy::ecs::world::CommandQueue;
use bevy::image::TextureAtlasBuilderError;
use bevy::scene::ron::de::from_str;
use bevy::scene::ron::ser::Serializer;
use bevy::tasks::{AsyncComputeTaskPool, IoTaskPool, Task, block_on, poll_once};
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

#[derive(Component, Deref, DerefMut)]
pub struct AtlasBuildJob(Task<CommandQueue>);

#[derive(Component, Deref, DerefMut)]
pub struct AtlasSaveJob(Task<CommandQueue>);

const GAME_CONFIG_FILE: &'static str = "config.ron";
const USER_CONFIG_FILE: &'static str = "userdata.ron";

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsTracking>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_atlas_map: ResMut<TextureAtlasHashMap>,
    folders: Res<Assets<LoadedFolder>>,
    images: Res<Assets<Image>>,
) -> Result<(), BevyError> {
    // static sprites
    for image_path in [
        "images/background.png",
        "images/bullet.png",
        "images/player.png",
        "images/enemy/basic_enemy.png",
    ] {
        let handle: Handle<Image> = asset_server.load(image_path);
        loading.add(handle.untyped());
    }

    // const ATLAS_LIST: [&'static str; 1] = ["player"];

    // let pool = IoTaskPool::get();

    // for atlas in ATLAS_LIST {
    //     let target_dir = Path::new("assets").join("images").join("atlas").join(atlas);
    //     let source_dir = Path::new("assets")
    //         .join("images")
    //         .join("pre_atlas")
    //         .join(atlas);
    //     let target_ron = target_dir.clone().with_extension("ron");
    //     let target_sheet = target_dir.with_extension("png");

    //     if !(target_sheet.exists() && target_ron.exists()) {
    //         // clobber and rebuild
    //         let handle = asset_server.load_folder(source_dir);
    //         let entity = commands.spawn_empty().id();
    //         let task = pool.spawn(async move {
    //             let mut command_queue = CommandQueue::default();

    //             command_queue.push(move |world: &mut World| {
    //                 let (asset_server, images) = {
    //                     let mut system_state =
    //                         SystemState::<(ResMut<AssetServer>, ResMut<Assets<Image>>)>::new(world);
    //                     let (asset_server, images) = system_state.get_mut(world);
    //                     (asset_server, images)
    //                 };

    //                 let folder = loop {
    //                     match asset_server.dependency_load_state(handle.id()) {
    //                         DependencyLoadState::NotLoaded | DependencyLoadState::Loading => {
    //                             async_std::task::sleep(Duration::from_millis(100)).await;
    //                         }

    //                         DependencyLoadState::Loaded => {
    //                             break folders.get(&handle).unwrap();
    //                         }
    //                         DependencyLoadState::Failed(asset_load_error) => {
    //                             panic!(
    //                                 "failed to load with error {}",
    //                                 asset_load_error.to_string()
    //                             );
    //                         }
    //                     }
    //                 };

    //                 let mut builder = TextureAtlasBuilder::default().auto_format_conversion(true);
    //                 for handle in folder.handles {
    //                     let texture = images.get(&handle.try_typed().unwrap()).unwrap();
    //                     builder.add_texture(None, texture);
    //                 }
    //                 let Ok((layout, sources, actual_atlas)) = builder.build() else {
    //                     warn!("failed to build texture atlas");
    //                     return;
    //                 };
    //                 let id = images.add(actual_atlas);

    //                 let save_task = pool.spawn(async move {
    //                     let mut command_queue = CommandQueue::default();
    //                     command_queue.push(move |world: &mut World| {
    //                         let images = {
    //                             let mut system_state =
    //                                 SystemState::<(Res<Assets<Image>>,)>::new(world);
    //                             let images = system_state.get_mut(world).0;
    //                             images
    //                         };
    //                         let image = images.get(&id).unwrap();

    //                     });
    //                     command_queue
    //                 });
    //                 world
    //                     .entity_mut(entity)
    //                     // Add our new `Mesh3d` and `MeshMaterial3d` to our tagged entity
    //                     .insert(AtlasSaveJob(save_task))
    //                     // Task is complete, so remove task component from entity
    //                     .remove::<AtlasBuildJob>();
    //             });
    //             command_queue
    //         });
    //         commands.entity(entity).insert(AtlasBuildJob(task));
    //     }
    // }
    // spritesheets
    // for sheet_layout_path in ["images/sheets/player.ron"] {
    //     let mut file = File::open(sheet_layout_path)?;
    //     let mut data = String::new();
    //     file.read_to_string(&mut data)?;

    //     // let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    //     let layout: TextureAtlasLayout = from_str(data.as_str())?;
    //     let texture_atlas_layout = texture_atlas_layouts.add(layout);
    //     texture_atlas_map.insert("player".to_string(), texture_atlas_layout);
    // }

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
    task_query: Query<&AtlasBuildJob>,
    game_config_asset: Res<Assets<GameConfig>>,
    user_data_asset: Res<Assets<UserData>>,
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
    if !task_query.is_empty() {
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
