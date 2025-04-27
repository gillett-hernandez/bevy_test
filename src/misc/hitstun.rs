use std::time::Duration;

use bevy::prelude::*;

use crate::{
    config::GameConfig,
    events::{EnemyHit, PlayerHit},
    gamestate::GameState,
};

#[derive(Copy, Clone, Deref, DerefMut, Default, Resource)]
pub struct HitStun(pub bool);

pub fn in_game_no_hitstun(state: Res<State<GameState>>, hitstun: Res<HitStun>) -> bool {
    state.get() == &GameState::InGame && !(**hitstun)
}

pub fn hitstun_trigger_system(
    mut player_hit_events: EventReader<PlayerHit>,
    mut enemy_hit_events: EventReader<EnemyHit>,
    mut gamestate: ResMut<NextState<GameState>>,
    mut hitstun: ResMut<HitStun>,
) {
    if !player_hit_events.is_empty() || !enemy_hit_events.is_empty() {
        player_hit_events.clear();
        enemy_hit_events.clear();
        **hitstun = true;
        gamestate.set(GameState::HitStun);
    }
}

pub fn hitstun_tick_system(
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    mut hitstun: ResMut<HitStun>,
    mut gamestate: ResMut<NextState<GameState>>,
    settings: Res<GameConfig>,
) {
    if let Some(timer) = timer.as_mut() {
        if timer.tick(time.delta()).finished() {
            timer.reset();
            **hitstun = false;
            gamestate.set(GameState::InGame);
        }
    } else {
        *timer = Some(Timer::new(
            Duration::from_millis(settings.hitstun_time_ms as u64),
            TimerMode::Once,
        ));
    }
}
