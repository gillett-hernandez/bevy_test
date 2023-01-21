use bevy::prelude::*;

use crate::config::Config;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    MainMenu,
    InHanger,
    InGame,
    Paused,
    // GameOver,
}

pub enum InputMode {
    Keyboard,
    Controller,
}

#[derive(Resource)]
pub struct Game {
    pub config: Config,
    pub input_mode: InputMode,
}
