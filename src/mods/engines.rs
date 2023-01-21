use bevy::prelude::*;

use crate::body_type_stats::PlaneMovementStats;

use super::Recalculated;

#[derive(Component, Default)]
pub struct NormalEngine(bool);

#[derive(Component, Default)]
pub struct SuperboostEngine {
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
    fn modify(&self, stats: &mut PlaneMovementStats) {
        // modifies turn speed when boosting

        stats.acceleration *= 2.0;
        if self.boosting {
            stats.turn_speed /= 2.0;
        } else {
            stats.turn_speed *= 2.0;
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
    fn modify(&self, stats: &mut PlaneMovementStats) {
        //pulse fires, needs unique system or something.
    }
}

// immune to water damage
#[derive(Component, Default)]
pub struct SubmarineEngine(bool);
