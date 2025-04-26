use bevy::prelude::*;

use crate::{enemy::Enemy, mods::guns::Bullet};
// use bevy::time::Timer;

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq, Debug, Hash, States, Default)]
pub enum GameState {
    #[default]
    Loading,    // can transition to mainmenu
    MainMenu,   // can transition to inhanger or ingame (quickstart)
    InHanger,   // can transition to ingame or main menu
    InGame,     // can transition to game ending and hitstun
    HitStun,    // can transition to ingame
    Paused,     // can transition to game ending and quitting
    GameEnding, // can transition to mainmenu or inhanger
    Quitting,   // quits the game, saving player data to disk and despawning all entities
}

#[derive(Resource, DerefMut, Deref)]
pub struct GameEndingTimer(pub Timer);

pub fn game_ending_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GameEndingTimer>,
    mut game_state: ResMut<NextState<GameState>>,
    enemy_query: Query<Entity, With<Enemy>>,
    bullet_query: Query<Entity, With<Bullet>>,
) {
    timer.tick(time.delta());
    for entity in enemy_query.iter().chain(bullet_query.iter()) {
        commands.entity(entity).despawn();
    }

    if timer.finished() {
        game_state.set(GameState::MainMenu);
        timer.reset();
    }
}
