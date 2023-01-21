use bevy::prelude::*;

use crate::{body_type_stats::PlaneMovementStats, player::Intent};

use super::Recalculated;

#[derive(Component, Default)]
pub struct NormalEngine(bool);

#[derive(Component, Default)]
pub struct SuperboostEngine {
    modified_acceleration: bool,
    dirty: bool,
    boosting: bool,
}

impl Recalculated<PlaneMovementStats> for SuperboostEngine {
    fn is_dirty(&self) -> bool {
        self.dirty
    }
    fn set_dirty(&mut self) {
        self.dirty = true;
    }
    fn clear_dirty(&mut self) {
        self.dirty = false;
    }
    fn modify(&mut self, stats: &mut PlaneMovementStats) {
        // modifies turn speed when boosting
        if !self.modified_acceleration {
            stats.acceleration *= 2.0;
            self.modified_acceleration = true;
        }
        if self.boosting {
            stats.turn_speed /= 3.0;
        } else {
            stats.turn_speed *= 3.0;
        }
    }
}

#[derive(Component, Default)]
pub struct GungineEngine(bool);

impl Recalculated<PlaneMovementStats> for GungineEngine {
    fn is_dirty(&self) -> bool {
        self.0
    }
    fn set_dirty(&mut self) {
        self.0 = true;
    }
    fn clear_dirty(&mut self) {
        self.0 = false;
    }
    fn modify(&mut self, stats: &mut PlaneMovementStats) {
        //pulse fires, needs unique system or something.
        todo!()
    }
}

// immune to water damage
#[derive(Component, Default)]
pub struct SubmarineEngine(bool);

pub fn superboost_engine_sync_system(
    mut query: Query<(&mut SuperboostEngine, &Intent), Changed<Intent>>,
) {
    for (mut engine, intent) in query.iter_mut() {
        if engine.boosting != intent.accelerate {
            engine.boosting = intent.accelerate;
            engine.set_dirty();
        }
    }
}
