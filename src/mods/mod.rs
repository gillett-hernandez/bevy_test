use bevy::prelude::*;

use crate::{
    body_type_stats::PlaneMovementStats, gamestate::GameState, misc::HP, player::PlayerStats,
};

pub mod body;
pub mod engines;
pub mod guns;

use body::*;
use engines::*;

pub trait Recalculated<Target: Component>: Component {
    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self);
    fn clear_dirty(&mut self);
    fn modify(&self, stats: &mut Target);
}

// when any component is set to dirty, it will recalculate its effect on playerstats.
// to correctly recalculate the playerstats, playerstats must be reset to the default and all components that have Recalculated must be set to dirty.
// this will be called infrequently, so it's fine if there's mutex-type locking that occurs for playerstats.
pub fn recalculate_stats_system<R, T>(mut query: Query<(&mut T, &mut R), Changed<T>>)
where
    R: Recalculated<T>,
    T: Component,
{
    for (mut stats, mut recalc) in query.iter_mut() {
        if recalc.is_dirty() {
            recalc.modify(stats.as_mut());
            recalc.clear_dirty();
        }
    }
}

pub struct BodyModsPlugin {}

impl Plugin for BodyModsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(recalculate_stats_system::<MeleeBody, HP>)
                .with_system(recalculate_stats_system::<MeleeBody, PlayerStats>)
                .with_system(recalculate_stats_system::<HeavyBody, HP>)
                .with_system(recalculate_stats_system::<HeavyBody, PlaneMovementStats>)
                .with_system(recalculate_stats_system::<SuperboostEngine, _>)
                .with_system(recalculate_stats_system::<GungineEngine, _>), // .with_system(recalculate_stats_system::<HeavyBody>)
        );
    }
}
