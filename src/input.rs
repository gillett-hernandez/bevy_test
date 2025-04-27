use bevy::prelude::*;

use crate::{player::Player, userdata::UserData};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub enum InputMode {
    #[default]
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

pub fn player_input_intent_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // axis: Res<Axis<GamepadAxis>>,
    // buttons_input: Res<ButtonInput<GamepadButton>>,
    userdata: Res<UserData>,
    mut query: Query<(Entity, &mut Intent), With<Player>>,
) -> Result<(), BevyError> {
    let (_entity, mut intent) = query.single_mut()?;

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

            if keyboard_input.pressed(KeyCode::ArrowUp) {
                // accelerate
                intent.accelerate = true;
            } else {
                intent.accelerate = false;
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                // decelerate
                intent.brake = true;
            } else {
                intent.brake = false;
            }

            intent.turn_intent = 0.0;

            if keyboard_input.pressed(KeyCode::ArrowRight) {
                // turn right
                intent.turn_intent -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                // turn left
                intent.turn_intent += 1.0;
            }
        }
        InputMode::Controller => {
            panic!();
        }
    }
    Ok(())
}
