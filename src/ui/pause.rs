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
    let esc_pressed = keyboard_input.just_pressed(KeyCode::Escape);
    let start_pressed = button_input.just_pressed(GamepadButton {
        gamepad: Gamepad::new(0),
        button_type: GamepadButtonType::Start,
    });
    if pause_debounce_timer.tick(time.delta()).finished() && (esc_pressed || start_pressed) {
        let _ = game_state.pop();
        pause_debounce_timer.reset();
    }
}

fn pause_input_handler(
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    time: Res<Time>,
    mut pause_debounce_timer: ResMut<PauseDebounceTimer>,
    mut game_state: ResMut<State<GameState>>,
) {
    let esc_pressed = keyboard_input.just_pressed(KeyCode::Escape);
    let start_pressed = button_input.just_pressed(GamepadButton {
        gamepad: Gamepad::new(0),
        button_type: GamepadButtonType::Start,
    });
    if pause_debounce_timer.tick(time.delta()).finished() && (esc_pressed || start_pressed) {
        game_state.push(GameState::Paused).unwrap();
        pause_debounce_timer.reset();
    }
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
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
