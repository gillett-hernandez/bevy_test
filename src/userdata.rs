use bevy::platform::collections::hash_map::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    input::InputMode,
    mods::{body::BodyType, engines::EngineType, guns::WeaponType},
};

#[derive(Resource, Asset, TypePath, Serialize, Deserialize, Clone)]
pub struct UserData {
    // player files
    // basically, a store of all the stuff that has been unlocked,
    // the player's high score and what build it was achieved with,
    // and what their currently selected build is
    pub selected_input_method: InputMode,
    pub unlockables: HashMap<String, bool>,
    pub high_score: (u32, String),
    // index of gun, body, and engine
    pub selected_build: (WeaponType, BodyType, EngineType),
    pub display_fps: bool,
    pub deadzone_radius: f32,
    pub desired_fps: u32,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            selected_input_method: InputMode::Keyboard,
            unlockables: HashMap::new(),
            high_score: (0, "".to_string()),
            selected_build: (
                WeaponType::default(),
                BodyType::default(),
                EngineType::default(),
            ),
            display_fps: true,
            deadzone_radius: 0.3,
            desired_fps: 60,
        }
    }
}
