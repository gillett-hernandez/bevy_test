use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl, AudioSource};

use crate::events::EnemyHit;

use super::Sfx;

pub fn enemy_hit_sound_effect_system(
    hit_events: EventReader<EnemyHit>,
    assets: Res<Assets<AudioSource>>,
    audio: Res<AudioChannel<Sfx>>,
) {
    // restart hit sound for playback, interrupting prior hit sounds
    if !hit_events.is_empty() {
        info!("found enemy hit event, playing sound");
        audio
            .play(assets.get_handle("sfx/hit_sound.ogg"))
            .with_volume(0.1);
        hit_events.clear();
    }
}
