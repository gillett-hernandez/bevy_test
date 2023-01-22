use std::time::Duration;

use bevy::prelude::*;

use crate::gamestate::GameState;

#[derive(Resource)]
struct PauseTimer(Timer);

fn pause_menu_system(
    _keyboard_input: Res<Input<KeyCode>>,
    _time: Res<Time>,
    _pause_timer: ResMut<PauseTimer>,
    _game_state: ResMut<State<GameState>>,
) {
    // TODO: implement pausing such that the camera switches to the UI camera and the player doesn't get recreated and stuff.
    // if pause_timer.0.tick(time.delta()).finished() {
    //     if keyboard_input.just_pressed(KeyCode::Escape) {
    //         game_state.set(GameState::InGame).unwrap();
    //     }
    //     pause_timer.0.reset();
    // }
}

fn pause_input_handler(
    _keyboard_input: Res<Input<KeyCode>>,
    _time: Res<Time>,
    _pause_timer: ResMut<PauseTimer>,
    _game_state: ResMut<State<GameState>>,
) {
    // TODO: implement pausing such that the camera switches to the UI camera and the player doesn't get recreated and stuff.
    // if pause_timer.0.tick(time.delta()).finished() {
    //     if keyboard_input.just_pressed(KeyCode::Escape) {
    //         game_state.set(GameState::Paused).unwrap();
    //     }
    //     pause_timer.0.reset();
    // }
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame).with_system(pause_input_handler),
        )
        .add_system_set(SystemSet::on_update(GameState::Paused).with_system(pause_menu_system))
        .insert_resource(PauseTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Once,
        )));
    }
}
