use bevy::{prelude::*, utils::HashMap};

use crate::{
    input::InputMode,
    mods::{body::BodyType, engines::EngineType, guns::WeaponType},
};

#[derive(Resource)]
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
}

impl UserData {
    pub fn new() -> Self {
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
        }
    }
}
