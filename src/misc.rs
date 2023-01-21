use std::{collections::HashMap, f32::consts::TAU, hash::Hash, time::Duration};

use bevy::prelude::*;
use rand::random;

use crate::{enemy::Enemy, gamestate::Game, physics::Physics, player::Player};

// misc functions

pub trait ToVec3: Sized {
    fn to_vec3(&self) -> Vec3;
}

impl ToVec3 for Vec2 {
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

pub fn project(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.y)
}

pub fn random_in_circle() -> Vec2 {
    let (u, v) = (random::<f32>(), random::<f32>());
    let phi = u * TAU;
    let r = v.sqrt();
    let (sin, cos) = phi.sin_cos();
    Vec2::new(r * cos, r * sin)
}

// lifetime

#[derive(Component)]
pub struct Lifetime {
    alive: bool,
    timer: Timer,
}

impl Lifetime {
    pub fn new(duration: Duration) -> Self {
        Lifetime {
            alive: true,
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

pub fn lifetime_system(time: Res<Time>, mut query: Query<&mut Lifetime>) {
    for mut lifetime in query.iter_mut() {
        if lifetime.timer.tick(time.delta()).just_finished() {
            lifetime.alive = false;
        }
    }
}

pub fn lifetime_postprocess_system(mut commands: Commands, query: Query<(Entity, &Lifetime)>) {
    for (entity, lifetime) in query.iter() {
        if !lifetime.alive {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

// timers component
// somewhat heaviweight, but allows for arbitrary timers on an entity, accessible through a kv-store

// #[derive(Component)]
// pub struct Timers<T>
// where
//     T: Eq + Hash,
// {
//     pub timers: HashMap<T, Timer>,
// }

// impl<T> Timers<T>
// where
//     T: Eq + Hash,
// {
//     pub fn new() -> Self {
//         Timers {
//             timers: HashMap::new(),
//         }
//     }

//     pub fn with_pair(mut self, k: T, v: Timer) -> Self {
//         self.timers.insert(k, v);
//         self
//     }
// }

// upper and lower bounds

#[derive(Component)]
pub struct VerticallyBounded;

pub fn vertical_bound_system(
    mut query: Query<(Entity, &mut Physics, &mut Transform), With<VerticallyBounded>>,
    game: Res<Game>,
    time: Res<Time>,
) {
    let strength = game.config.vertical_bounds_rotation_speed * time.delta_seconds();
    let deadzone_width = 0.1; // to calculate from radians, do sin(d/2) where d is the deadzone width in radians.
                              // however for small x, sin(x) ~= x
                              // thus the deadzone width in radians is approximately 2 times the variable as written.
    for (_e, mut physics, mut transform) in query.iter_mut() {
        if transform.translation.y > game.config.upper_bound {
            // handle upper bound
            // should rotate player towards down, and push them down as well
            let current_pointing_direction = transform.rotation * Vec3::Y;
            if current_pointing_direction.y < 0.0 {
                // currently pointing down
                if current_pointing_direction.x <= -deadzone_width {
                    // rotate to the left
                    transform.rotation *= Quat::from_rotation_z(strength);
                } else if current_pointing_direction.x > deadzone_width {
                    // rotate to the right
                    transform.rotation *= Quat::from_rotation_z(-strength);
                }
                // else do nothing.
            } else {
                // currently pointing up
                if current_pointing_direction.x <= 0.0 {
                    // rotate to the left
                    transform.rotation *= Quat::from_rotation_z(strength);
                } else {
                    // rotate to the right
                    transform.rotation *= Quat::from_rotation_z(-strength);
                }
            }
            physics.velocity.y -= game.config.upper_repulsion_strength;
        } else if transform.translation.y < game.config.lower_bound {
            // handle lower bound
            // should rotate player towards up, and push them up as well
            let current_pointing_direction = transform.rotation * Vec3::Y;
            if current_pointing_direction.y > 0.0 {
                // currently pointing up
                if current_pointing_direction.x <= -deadzone_width {
                    // rotate to the left
                    transform.rotation *= Quat::from_rotation_z(-strength);
                } else if current_pointing_direction.x > deadzone_width {
                    // rotate to the right
                    transform.rotation *= Quat::from_rotation_z(strength);
                }
                // else do nothing
            } else {
                // currently pointing down
                if current_pointing_direction.x <= 0.0 {
                    // rotate to the left
                    transform.rotation *= Quat::from_rotation_z(-strength);
                } else {
                    // rotate to the right
                    transform.rotation *= Quat::from_rotation_z(strength);
                }
            }
            physics.velocity.y += game.config.lower_repulsion_strength;
        } else {
            continue;
        }
    }
}

#[derive(Component)]
pub struct TakesContactDamage {}

#[derive(Component)]
pub struct DealsContactDamage {}

pub fn contact_damage_system<Takers: Component, Dealers: Component>(
    mut query_damage_takers: Query<(Entity, &mut Takers, &TakesContactDamage)>,
    query_damage_dealers: Query<(Entity, &Dealers, &DealsContactDamage)>,
) {
    // check collisions between takers and dealers.
    // ideally we would have an acceleration structure here to make collision checks faster
}

#[derive(Component)]
pub struct HP {
    pub hp: f32,
    pub max: f32,
    pub regen: f32,
}

pub fn hp_regen_system(mut query: Query<&mut HP>) {
    for mut hp in query.iter_mut() {
        hp.hp += hp.regen;
        if hp.hp > hp.max {
            hp.hp = hp.max;
        }
    }
}
