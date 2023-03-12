use bevy::prelude::*;

use std::fmt::Debug;

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
    fn modify(&mut self, stats: &mut Target);
}

// when any component is set to dirty, it will recalculate its effect on playerstats.
pub fn recalculate_stats_system<R, T>(mut query: Query<(&mut T, &mut R), Changed<R>>)
where
    R: Recalculated<T>,
    T: Component + Debug,
{
    for (mut stats, mut recalc) in query.iter_mut() {
        if recalc.is_dirty() {
            // print!("just modified stats from {:?}", stats);
            recalc.modify(stats.as_mut());
            // info!("to {:?}", stats);
            recalc.clear_dirty();
        }
    }
}

pub struct BodyModsPlugin;

impl Plugin for BodyModsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                recalculate_stats_system::<MeleeBody, HP>,
                recalculate_stats_system::<MeleeBody, PlayerStats>,
                recalculate_stats_system::<HeavyBody, HP>,
                recalculate_stats_system::<HeavyBody, PlaneMovementStats>,
                recalculate_stats_system::<SuperboostEngine, _>,
                recalculate_stats_system::<GungineEngine, _>,
                superboost_engine_sync_system,
            )
                .in_set(OnUpdate(GameState::InGame)), // .with_system(recalculate_stats_system::<HeavyBody>)
                                                      // .with_system(gungine_sync_system)
        );
    }
}
