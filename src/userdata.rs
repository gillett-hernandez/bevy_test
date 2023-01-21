use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct UserData {
    // player files
    // basically, a store of all the stuff that has been unlocked,
    // the player's high score and what build it was achieved with,
    // and what their currently selected build is
    pub unlockables: HashMap<String, bool>,
    pub high_score: (u32, String),
    // index of gun, body, and engine
    pub selected_build: (u8, u8, u8),
}

impl UserData {
    pub fn new() -> Self {
        Self {
            unlockables: HashMap::new(),
            high_score: (0, "".to_string()),
            selected_build: (0, 0, 0),
        }
    }
}
