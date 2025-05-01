use crate::{
    gamestate::GameState,
    input::InputMode,
    mods::{engines::EngineType, guns::WeaponType},
    userdata::UserData,
};
use bevy::prelude::*;

#[derive(Resource, DerefMut, Deref)]
pub struct MainMenuDebounceTimer(pub Timer);

pub fn setup_main_menu_ui() {}

pub fn main_menu_ui_system(
    time: Res<Time>,
    mut timer: ResMut<MainMenuDebounceTimer>,
    mut state: ResMut<NextState<GameState>>,
    // mut data: ResMut<UserData>,
) {
    // do ui stuff, but placeholder for now

    // with no ui stuff and choices to make, should enter the in-game state after 1 frame

    timer.tick(time.delta());
    if timer.finished() {
        state.set(GameState::InGame);
        timer.reset();
    }
}
