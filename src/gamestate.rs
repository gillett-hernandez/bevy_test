use bevy::prelude::*;

use crate::{enemy::Enemy, mods::guns::Bullet, player::Player};
// use bevy::time::Timer;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,    // can transition to mainmenu
    MainMenu,   // can transition to inhanger or ingame (quickstart)
    InHanger,   // can transition to ingame or main menu
    InGame,     // can transition to game ending
    Paused,     // can transition to game ending and quitting
    GameEnding, // can transition to mainmenu or inhanger
    Quitting,   // quits the game, saving player data to disk and despawning all entities
}

#[derive(Resource)]
pub struct GameEndingTimer(pub Timer);

pub fn game_ending_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GameEndingTimer>,
    mut game_state: ResMut<State<GameState>>,
    enemy_query: Query<Entity, With<Enemy>>,
    bullet_query: Query<Entity, With<Bullet>>,
) {
    timer.0.tick(time.delta());
    for entity in enemy_query.iter().chain(bullet_query.iter()) {
        commands.entity(entity).despawn_recursive();
    }

    if timer.0.finished() {
        let _ = game_state.set(GameState::MainMenu);
        timer.0.reset();
    }
}
