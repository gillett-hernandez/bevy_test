use bevy::prelude::*;

use crate::{enemy::Enemy, events::EnemyDeath};

#[derive(Resource)]
pub struct ComboCounter {
    decay_timer: Timer,
    count: usize,
}

impl ComboCounter {
    pub fn new(timer: Timer) -> Self {
        ComboCounter {
            decay_timer: timer,
            count: 0,
        }
    }
    pub fn multiplier_for_count(&self) -> f32 {
        1.0 + self.count as f32
    }
}

pub fn combo_enemy_death_subscriber(
    time: Res<Time>,
    mut combo: ResMut<ComboCounter>,
    mut events: EventReader<EnemyDeath>,
) {
    if combo.decay_timer.tick(time.delta()).just_finished() {
        // timer finished
        if events.is_empty() {
            combo.decay_timer.reset();
            combo.count = 0;
        } else {
            combo.decay_timer.reset();
        }
    }
    for _ in events.iter() {
        combo.count += 1;
    }
}
