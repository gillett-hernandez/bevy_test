use bevy::prelude::*;

use crate::{events::EnemyDeath, gamestate::GameState};

use super::combo::ComboCounter;

#[derive(Resource, Deref, DerefMut)]
pub struct ScoreTracker(pub usize);

pub fn score_system(
    mut score_tracker: ResMut<ScoreTracker>,
    mut enemy_deaths: EventReader<EnemyDeath>,
    combo: Res<ComboCounter>,
) {
    for enemy_death in enemy_deaths.read() {
        **score_tracker += (enemy_death.score as f32 * combo.multiplier()) as usize;
        info!("score is now {}", **score_tracker);
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScoreTracker(0))
            .add_systems(Update, score_system.run_if(in_state(GameState::InGame)));
    }
}
