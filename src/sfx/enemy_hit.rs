use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl, AudioSource};

use crate::events::EnemyHit;

use super::SFX;

pub fn enemy_hit_sound_effect_system(
    mut hit_events: EventReader<EnemyHit>,
    assets: Res<Assets<AudioSource>>,
    audio: Res<AudioChannel<SFX>>,
) {
    // restart hit sound for playback, interrupting prior hit sounds
    for event in hit_events.iter() {
        audio.play(assets.get_handle("enemy_hit.ogg"));
    }
}
