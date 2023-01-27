
use bevy::prelude::*;

use crate::{player::Player, input::Intent};

#[derive(Component, Debug)]
pub struct HP {
    pub hp: f32,
    pub max: f32,
    pub regen: f32,
}

pub fn hp_regen_system(mut query: Query<(&mut HP, Option<&Player>, Option<&Intent>)>) {
    for (mut hp, player, intent) in query.iter_mut() {
        if hp.hp < hp.max {
            if intent.map(|i| !i.fire).unwrap_or(true) {
                hp.hp += hp.regen;
            }
        } else if hp.hp > hp.max {
            hp.hp = hp.max;
        }
    }
}
