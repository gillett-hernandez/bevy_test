use bevy::prelude::*;
use bevy_kira_audio::AudioApp;

use crate::gamestate::GameState;

mod enemy_hit;

use enemy_hit::enemy_hit_sound_effect_system;

#[derive(Resource)]
pub struct Sfx;

impl Plugin for Sfx {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<Sfx>().add_system(enemy_hit_sound_effect_system.in_set(OnUpdate(GameState::InGame)));
    }
}
