use bevy::prelude::*;

pub mod bullet;
pub mod gun_collection;
pub mod laser;

pub use bullet::{
    enemy_bullet_collision_system, player_bullet_collision_system, Bullet, BulletCollisionPlugin,
};
pub use gun_collection::GunCollectionPlugin;
pub use laser::{Laser, LaserCollisionPlugin};

use crate::gamestate::GameState;

pub struct WeaponSubsystemPlugin;

impl Plugin for WeaponSubsystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_bullet_collision_system)
                .with_system(enemy_bullet_collision_system),
        );
    }
}
