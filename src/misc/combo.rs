use bevy::prelude::*;

use crate::{events::EnemyDeath};

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
    if combo.count > 0 {
        combo.decay_timer.tick(time.delta());
    }
    if combo.decay_timer.just_finished() {
        // timer finished
        if events.is_empty() {
            println!("combo reset");
            combo.decay_timer.reset();
            combo.count = 0;
        }
    }
    for _ in events.iter() {
        combo.count += 1;
        combo.decay_timer.reset();
        println!("combo is at {}", combo.count);
    }
}
