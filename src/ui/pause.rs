use std::time::Duration;

use bevy::prelude::*;

use crate::gamestate::GameState;

#[derive(Resource, DerefMut, Deref)]
struct PauseDebounceTimer(Timer);

fn pause_menu_system(
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    time: Res<Time>,
    mut pause_debounce_timer: ResMut<PauseDebounceTimer>,
    mut game_state: ResMut<State<GameState>>,
) {
    // TODO: implement pausing such that the camera switches to the UI camera and the player doesn't get recreated and stuff.
    if pause_debounce_timer.tick(time.delta()).finished() {
        if keyboard_input.pressed(KeyCode::Escape)
            || button_input.pressed(GamepadButton {
                gamepad: Gamepad::new(0),
                button_type: GamepadButtonType::Start,
            })
        {
            let _ = game_state.pop();
            pause_debounce_timer.reset();
        }
    }
}

fn pause_input_handler(
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    time: Res<Time>,
    mut pause_debounce_timer: ResMut<PauseDebounceTimer>,
    mut game_state: ResMut<State<GameState>>,
) {
    // TODO: implement pausing such that the camera switches to the UI camera and the player doesn't get recreated and stuff.
    if pause_debounce_timer.tick(time.delta()).finished() {
        if keyboard_input.pressed(KeyCode::Escape)
            || button_input.pressed(GamepadButton {
                gamepad: Gamepad::new(0),
                button_type: GamepadButtonType::Start,
            })
        {
            let _ = game_state.push(GameState::Paused).unwrap();
            pause_debounce_timer.reset();
        }
    }
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        println!("adding pause plugin");
        app.add_system_set(
            SystemSet::on_update(GameState::InGame).with_system(pause_input_handler),
        )
        .add_system_set(SystemSet::on_update(GameState::Paused).with_system(pause_menu_system))
        .insert_resource(PauseDebounceTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Once,
        )));
    }
}
