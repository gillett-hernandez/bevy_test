use bevy::prelude::*;
// use bevy_kira_audio::AudioApp;

use crate::gamestate::GameState;

mod enemy_hit;

use enemy_hit::enemy_hit_sound_effect_system;

#[derive(Component)]
pub struct Sfx;

impl Plugin for Sfx {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            enemy_hit_sound_effect_system.run_if(in_state(GameState::InGame)),
        );
    }
}
