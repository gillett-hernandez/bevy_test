use bevy::prelude::*;

use crate::{input::Intent, player::Player};

#[derive(Component, Debug)]
pub struct HP {
    pub hp: f32,
    pub max: f32,
    pub regen: f32,
}

pub fn hp_regen_system(
    time: Res<Time>,
    mut query: Query<(&mut HP, Option<&Player>, Option<&Intent>)>,
) {
    for (mut hp, maybe_player, intent) in query.iter_mut() {
        if hp.hp < hp.max {
            // only heal while not firing
            if intent.map(|i| !i.fire).unwrap_or(true) {
                hp.hp += hp.regen * time.delta_secs();
            }
        } else if hp.hp > hp.max {
            hp.hp = hp.max;
        }
    }
}
