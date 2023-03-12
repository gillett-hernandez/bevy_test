use std::time::Duration;

use bevy::prelude::*;

use crate::{
    events::{EnemyHit, PlayerHit},
    gamestate::GameState,
};

pub fn hitstun_trigger_system(
    player_hit_events: EventReader<PlayerHit>,
    enemy_hit_events: EventReader<EnemyHit>,
    mut gamestate: ResMut<NextState<GameState>>,
) {
    if !player_hit_events.is_empty() || !enemy_hit_events.is_empty() {
        player_hit_events.clear();
        enemy_hit_events.clear();
        // gamestate.push(GameState::HitStun);
    }
}

pub fn hitstun_tick_system(
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    mut gamestate: ResMut<NextState<GameState>>,
) {
    if let Some(timer) = timer.as_mut() {
        if timer.tick(time.delta()).finished() {
            // gamestate.pop();
            timer.reset();
        }
    } else {
        *timer = Some(Timer::new(Duration::from_millis(20), TimerMode::Once));
    }
}
