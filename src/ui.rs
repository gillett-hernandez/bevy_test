use crate::{gamestate::GameState, userdata::UserData};
use bevy::prelude::*;

pub fn setup_main_menu_ui() {}

pub fn main_menu_ui_system(mut state: ResMut<State<GameState>>, mut data: ResMut<UserData>) {
    // do ui stuff, but placeholder for now

    // with no ui stuff and choices to make, should enter the in-game state after 1 frame
    data.selected_build.2 = 1;

    let _ = state.set(GameState::InGame);
}
