use bevy::prelude::*;

use crate::{player::Player, userdata::UserData};

pub enum InputMode {
    Keyboard,
    Controller,
}

#[derive(Component, Default)]
pub struct Intent {
    // distilled input
    pub accelerate: bool,
    pub brake: bool,
    pub turn_intent: f32,
    pub fire: bool,
    pub just_fired: bool, // fired on this frame
}

pub fn player_intent_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    axis: Res<Axis<GamepadAxis>>,
    buttons_input: Res<Input<GamepadButton>>,
    userdata: Res<UserData>,
    mut query: Query<(Entity, &mut Intent), With<Player>>,
) {
    let (_entity, mut intent) = query.single_mut();

    match userdata.selected_input_method {
        InputMode::Keyboard => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                intent.just_fired = true;
            } else {
                intent.just_fired = false;
            }
            if keyboard_input.pressed(KeyCode::Space) {
                intent.fire = true;
            } else {
                intent.fire = false;
            }

            if keyboard_input.pressed(KeyCode::Up) {
                // accelerate
                intent.accelerate = true;
            } else {
                intent.accelerate = false;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                // decelerate
                intent.brake = true;
            } else {
                intent.brake = false;
            }

            intent.turn_intent = 0.0;

            if keyboard_input.pressed(KeyCode::Right) {
                // turn right
                intent.turn_intent -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                // turn left
                intent.turn_intent += 1.0;
            }
        }
        InputMode::Controller => {
            let gamepad0 = Gamepad::new(0);
            if buttons_input.just_pressed(GamepadButton {
                gamepad: gamepad0,
                button_type: GamepadButtonType::South,
            }) || buttons_input.just_pressed(GamepadButton {
                gamepad: gamepad0,
                button_type: GamepadButtonType::RightTrigger,
            }) {
                intent.just_fired = true;
            } else {
                intent.just_fired = false;
            }
            if buttons_input.pressed(GamepadButton {
                gamepad: gamepad0,
                button_type: GamepadButtonType::South,
            }) || buttons_input.pressed(GamepadButton {
                gamepad: gamepad0,
                button_type: GamepadButtonType::RightTrigger,
            }) {
                intent.fire = true;
            } else {
                intent.fire = false;
            }

            if buttons_input.pressed(GamepadButton {
                gamepad: gamepad0,
                button_type: GamepadButtonType::RightTrigger2,
            }) {
                // accelerate
                intent.accelerate = true;
            } else {
                intent.accelerate = false;
            }
            if buttons_input.pressed(GamepadButton {
                gamepad: gamepad0,
                button_type: GamepadButtonType::LeftTrigger2,
            }) {
                // decelerate
                intent.brake = true;
            } else {
                intent.brake = false;
            }

            intent.turn_intent = 0.0;

            let left_axis_x_value = axis
                .get(GamepadAxis::new(gamepad0, GamepadAxisType::LeftStickX))
                .unwrap();
            if left_axis_x_value < -userdata.deadzone_radius {
                // turn right
                intent.turn_intent += 1.0;
            }
            if left_axis_x_value > userdata.deadzone_radius {
                // turn left
                intent.turn_intent -= 1.0;
            }
        }
    }
}
