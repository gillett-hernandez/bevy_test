use bevy::prelude::*;

use crate::events::EnemyHit;

use super::Sfx;

pub fn enemy_hit_sound_effect_system(
    mut hit_events: EventReader<EnemyHit>,
    query_sfx: Query<&AudioSink, With<Sfx>>,
) {
    // restart hit sound for playback, interrupting prior hit sounds
    if !hit_events.is_empty() {
        info!("found enemy hit event, playing sound");
        let Ok(q) = query_sfx.single() else {
            return;
        };
        q.play();
        hit_events.clear();
    }
}
