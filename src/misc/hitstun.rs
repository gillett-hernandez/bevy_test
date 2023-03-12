use std::time::Duration;

use bevy::prelude::*;

use crate::{
    events::{EnemyHit, PlayerHit},
    gamestate::GameState,
};

#[derive(Copy, Clone, Deref, DerefMut, Default, Resource)]
pub struct HitStun(pub bool);

pub fn in_game_no_hitstun(state: Res<State<GameState>>, hitstun: Res<HitStun>) -> bool {
    state.0 == GameState::InGame && !(**hitstun)
}

pub fn hitstun_trigger_system(
    mut player_hit_events: EventReader<PlayerHit>,
    mut enemy_hit_events: EventReader<EnemyHit>,
    // mut gamestate: ResMut<NextState<GameState>>,
    mut hitstun: ResMut<HitStun>,
) {
    if !player_hit_events.is_empty() || !enemy_hit_events.is_empty() {
        player_hit_events.clear();
        enemy_hit_events.clear();
        **hitstun = true;
        // gamestate.push(GameState::HitStun);
    }
}

pub fn hitstun_tick_system(
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    mut hitstun: ResMut<HitStun>,
) {
    if let Some(timer) = timer.as_mut() {
        if timer.tick(time.delta()).finished() {
            timer.reset();
            **hitstun = false;
            // gamestate.pop();
        }
    } else {
        *timer = Some(Timer::new(Duration::from_millis(20), TimerMode::Once));
    }
}
