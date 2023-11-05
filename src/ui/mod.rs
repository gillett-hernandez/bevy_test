use std::time::Duration;

use bevy::prelude::*;
// pub mod hud;
pub mod main;
pub mod pause;

// pub use hud::*;
pub use main::*;
pub use pause::*;

use crate::gamestate::GameState;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PausePlugin)
            .insert_resource(MainMenuDebounceTimer(Timer::new(
                Duration::from_millis(500),
                TimerMode::Once,
            )))
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu_ui)
            .add_systems(
                Update,
                main_menu_ui_system.run_if(in_state(GameState::MainMenu)),
            );
    }
}
