use bevy::prelude::*;

mod enemy_hit;
pub mod hp;

use crate::gamestate::GameState;

use enemy_hit::enemy_hit_effect_system;
use hp::{hp_effect_setup_system, hp_effect_system};

#[derive(Component)]
pub struct Particle;

pub struct FxPlugin;

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame).with_system(hp_effect_setup_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(hp_effect_system)
                .with_system(enemy_hit_effect_system),
        );
    }
}
