use bevy::prelude::*;

use crate::{body_type_stats::PlaneMovementStats, misc::HP, player::PlayerStats};
use serde::{Deserialize, Serialize};

use super::Recalculated;


#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, Clone)]
pub enum BodyType {
    #[default]
    Normal,
    Heavy,
    Melee,
    Nuke,
    Bomber,
}

#[derive(Component, Default)]
pub struct NormalBody;

#[derive(Component)]
pub struct HeavyBody {
    dirty_plane: bool,
    dirty_hp: bool,
}

impl Default for HeavyBody {
    fn default() -> Self {
        Self {
            dirty_plane: true,
            dirty_hp: true,
        }
    }
}

impl Recalculated<PlaneMovementStats> for HeavyBody {
    fn is_dirty(&self) -> bool {
        self.dirty_plane
    }
    fn set_dirty(&mut self) {
        self.dirty_plane = true;
    }
    fn clear_dirty(&mut self) {
        self.dirty_plane = false;
    }
    fn modify(&mut self, stats: &mut PlaneMovementStats) {
        // modifies turn speed when firing, needs unique system or something.

        stats.acceleration *= 0.5;
        stats.turn_speed *= 0.5;
    }
}

impl Recalculated<HP> for HeavyBody {
    fn is_dirty(&self) -> bool {
        self.dirty_hp
    }
    fn set_dirty(&mut self) {
        self.dirty_hp = true;
    }
    fn clear_dirty(&mut self) {
        self.dirty_hp = false;
    }
    fn modify(&mut self, stats: &mut HP) {
        // modifies turn speed when firing, needs unique system or something.
        stats.max *= 3.0;
        stats.regen *= 0.6;
    }
}

#[derive(Component)]
pub struct MeleeBody {
    dirty_player: bool,
    dirty_hp: bool,
}

impl Default for MeleeBody {
    fn default() -> Self {
        Self {
            dirty_player: true,
            dirty_hp: true,
        }
    }
}

impl Recalculated<PlayerStats> for MeleeBody {
    fn is_dirty(&self) -> bool {
        self.dirty_player
    }
    fn set_dirty(&mut self) {
        self.dirty_player = true;
    }
    fn clear_dirty(&mut self) {
        self.dirty_player = false;
    }
    fn modify(&mut self, stats: &mut PlayerStats) {
        stats.takes_contact_damage = false;
        stats.contact_damage *= 2.0;
    }
}

impl Recalculated<HP> for MeleeBody {
    fn is_dirty(&self) -> bool {
        self.dirty_hp
    }
    fn set_dirty(&mut self) {
        self.dirty_hp = true;
    }
    fn clear_dirty(&mut self) {
        self.dirty_hp = false;
    }
    fn modify(&mut self, stats: &mut HP) {
        stats.max /= 2.0;
    }
}

// immune to water damage
#[derive(Component)]
pub struct NukeBody {
    dirty_player: bool,
    dirty_hp: bool,
}

impl Default for NukeBody {
    fn default() -> Self {
        Self {
            dirty_player: true,
            dirty_hp: true,
        }
    }
}

impl Recalculated<PlayerStats> for NukeBody {
    fn is_dirty(&self) -> bool {
        self.dirty_player
    }
    fn set_dirty(&mut self) {
        self.dirty_player = true;
    }
    fn clear_dirty(&mut self) {
        self.dirty_player = false;
    }
    fn modify(&mut self, stats: &mut PlayerStats) {
        stats.contact_damage *= 2.5;
    }
}
impl Recalculated<HP> for NukeBody {
    fn is_dirty(&self) -> bool {
        self.dirty_hp
    }
    fn set_dirty(&mut self) {
        self.dirty_hp = true;
    }
    fn clear_dirty(&mut self) {
        self.dirty_hp = false;
    }
    fn modify(&mut self, stats: &mut HP) {
        stats.max *= 1.5;
    }
}
