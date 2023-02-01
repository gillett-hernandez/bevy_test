use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl, AudioSource};

use crate::events::EnemyHit;

use super::Sfx;

pub fn enemy_hit_sound_effect_system(
    mut hit_events: EventReader<EnemyHit>,
    assets: Res<Assets<AudioSource>>,
    audio: Res<AudioChannel<Sfx>>,
) {
    // restart hit sound for playback, interrupting prior hit sounds
    if !hit_events.is_empty() {
        println!("found enemy hit event, playing sound");
        for _ in hit_events.iter() {
            audio.play(assets.get_handle("enemy_hit.ogg"));
        }
    }
}

