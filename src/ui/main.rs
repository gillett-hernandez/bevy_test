use crate::{gamestate::GameState, input::InputMode, userdata::UserData};
use bevy::prelude::*;

#[derive(Resource, DerefMut, Deref)]
pub struct MainMenuDebounceTimer(pub Timer);

pub fn setup_main_menu_ui() {}

pub fn main_menu_ui_system(
    time: Res<Time>,
    mut timer: ResMut<MainMenuDebounceTimer>,
    mut state: ResMut<State<GameState>>,
    mut data: ResMut<UserData>,
) {
    // do ui stuff, but placeholder for now

    // with no ui stuff and choices to make, should enter the in-game state after 1 frame

    timer.tick(time.delta());
    if timer.finished() {
        data.selected_build.2 = 1;
        data.selected_input_method = InputMode::Keyboard;
        let _ = state.set(GameState::InGame);
        timer.reset();
    }
}
