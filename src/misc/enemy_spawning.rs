use bevy::prelude::*;

use crate::{
    enemy::{add_basic_enemy, Enemy},
    events::{EnemyDeath, PlayerDeath},
    player::Player,
};

// wave system

#[derive(Resource)]
pub struct HeatTracker {
    // waves should spawn more frequently when heat is high.
    // waves should advance through a few archetypes, where early waves only spawn basic enemies and further waves spawn strong enemies.
    time_since_last_wave: f32,
    heat: f32,
    spawned_waves: u32,
}

impl Default for HeatTracker {
    fn default() -> Self {
        HeatTracker {
            // TODO: marking this as the place where the timing for the first wave is currently implemented.
            time_since_last_wave: 55.0,
            heat: 1.0,
            spawned_waves: 0,
        }
    }
}

impl HeatTracker {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

pub fn heat_player_death_subscriber(
    mut heat_tracker: ResMut<HeatTracker>,
    mut events: EventReader<PlayerDeath>,
) {
    if !events.is_empty() {
        events.clear();
        heat_tracker.reset();
    }
}

pub fn heat_enemy_death_subscriber(
    mut commands: Commands,
    mut heat_tracker: ResMut<HeatTracker>,
    mut events: EventReader<EnemyDeath>,
    query: Query<(Entity, &Enemy)>,
) {
    for event in events.read() {
        // make sure this enemy has not already been despawned for some reason.
        if query.contains(event.entity) {
            // spawn fx for death
            // queue sound playing
            // despawn enemy
            commands.entity(event.entity).despawn_recursive();
            // handle `heat`
            heat_tracker.heat += event.heat;
        }
    }
}

pub fn wave_system(
    mut commands: Commands,
    time: Res<Time>,
    mut heat_tracker: ResMut<HeatTracker>,
    player: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let player_position = player.single(); // assumes there's only one player.
    if heat_tracker.time_since_last_wave > 60.0 / heat_tracker.heat {
        // spawn wave
        // enemies need to be relatively close to the player.

        match heat_tracker.spawned_waves {
            // 0..=4 => {
            // spawn enemies based on this archetype.
            // },
            _ => {
                for _ in 0..10 {
                    add_basic_enemy(&mut commands, &asset_server, player_position.translation);
                }
            }
        }
        heat_tracker.spawned_waves += 1;

        // reset time and "lower" heat
        heat_tracker.time_since_last_wave = 0.0;
        heat_tracker.heat -= 0.01;
        if heat_tracker.heat < 1.0 {
            heat_tracker.heat = 1.0;
        }
    } else {
        heat_tracker.time_since_last_wave += time.delta_seconds();
    }
}
