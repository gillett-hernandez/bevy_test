use bevy::prelude::*;

use crate::{body_type_stats::PlaneMovementStats, player::Intent};

use super::Recalculated;

#[derive(Component, Default)]
pub struct NormalEngine(bool);

#[derive(Component)]
pub struct SuperboostEngine {
    modified_acceleration: bool,
    dirty: bool,
    boosting: bool,
    pub acceleration_modifier: f32,
    pub turn_speed_modifier: f32,
}

impl SuperboostEngine {
    pub fn new(acceleration_modifier: f32, turn_speed_modifier: f32) -> Self {
        Self {
            modified_acceleration: false,
            dirty: true,
            boosting: false,
            acceleration_modifier,
            turn_speed_modifier,
        }
    }
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
            stats.acceleration *= self.acceleration_modifier;
            self.modified_acceleration = true;
        }
        if self.boosting {
            stats.turn_speed *= self.turn_speed_modifier;
        } else {
            stats.turn_speed /= self.turn_speed_modifier;
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
