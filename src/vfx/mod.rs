use std::{f32::consts::PI, time::Duration};

use bevy::{prelude::*, sprite::Material2dPlugin};
use rand::random;

mod enemy_hit;
pub mod hp;

use crate::{gamestate::GameState, misc::Lifetime, physics::Physics};

use enemy_hit::enemy_hit_effect_system;
use hp::{hp_effect_setup_system, hp_effect_system};

#[derive(Component)]
pub struct Particle;

#[derive(Bundle)]
pub struct ParticleBundle {
    particle: Particle,
    #[bundle]
    spatial: SpatialBundle,
    physics: Physics,
    lifetime: Lifetime,
}

impl ParticleBundle {
    pub fn new(
        source_transform: &Transform,
        source_velocity: Vec3,
        velocity_variation: f32,
        particle_duration_seconds: f32,
    ) -> Self {
        // generates a new particlebundle with a velocity matching source_velocity, plus a disc uniformly distributed random velocity
        let (r1, r2) = (random::<f32>(), random::<f32>());
        let theta = 2.0 * PI * r1;
        let r = r2.sqrt() * velocity_variation;
        let (sin, cos) = theta.sin_cos();
        ParticleBundle {
            particle: Particle,
            spatial: SpatialBundle::from_transform(source_transform.clone()),
            physics: Physics {
                mass: 0.01,
                velocity: source_velocity + Vec3::new(r * cos, r * sin, 0.0),
                friction: 0.999,
                gravity: Vec3::new(0.0, -2.0, 0.0),
            },
            lifetime: Lifetime::new(Duration::from_secs_f32(particle_duration_seconds)),
        }
    }
}

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<hp::CustomMaterial>::default())
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(hp_effect_setup_system)
                    .with_system(hp_effect_system)
                    .with_system(enemy_hit_effect_system),
            );
    }
}
