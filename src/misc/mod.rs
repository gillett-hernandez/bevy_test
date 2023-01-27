use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::random;

use crate::gamestate::GameState;

mod combo;
mod contact_damage;
mod enemy_spawning;
mod hp;
mod lifetime;
mod score;
mod vertical_bound;

pub use enemy_spawning::HeatTracker;
pub use hp::{hp_regen_system, HP};
pub use lifetime::{lifetime_postprocess_system, lifetime_system, Lifetime};
pub use vertical_bound::{vertical_bound_system, VerticallyBounded};

use self::{
    combo::{combo_enemy_death_subscriber, ComboCounter},
    enemy_spawning::{heat_enemy_death_subscriber, heat_player_death_subscriber, wave_system},
};

// misc functions

pub trait ToVec3: Sized {
    fn to_vec3(&self) -> Vec3;
}

impl ToVec3 for Vec2 {
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

pub fn random_in_circle() -> Vec2 {
    let (u, v) = (random::<f32>(), random::<f32>());
    let phi = u * TAU;
    let r = v.sqrt();
    let (sin, cos) = phi.sin_cos();
    Vec2::new(r * cos, r * sin)
}

// fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
//     for e in &query {
//         commands.entity(e).despawn_recursive();
//     }
// }

pub struct MiscPlugin;
impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeatTracker>()
            .insert_resource(ComboCounter::new(Timer::from_seconds(2.0, TimerMode::Once)))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(wave_system)
                    .with_system(heat_player_death_subscriber)
                    .with_system(heat_enemy_death_subscriber)
                    .with_system(combo_enemy_death_subscriber),
            );
    }
}