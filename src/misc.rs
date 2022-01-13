use std::time::Duration;

use bevy::prelude::*;

use crate::{physics::Physics, Game};

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
            timer: Timer::new(duration, false),
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

// upper and lower bounds

#[derive(Component)]
pub struct VerticallyBounded {}

pub fn vertical_bound_system(
    mut query: Query<(Entity, &mut Physics, &mut Transform), With<VerticallyBounded>>,
    game: Res<Game>,
) {
    let strength = 0.04;
    for (_e, mut physics, mut transform) in query.iter_mut() {
        if transform.translation.y > game.config.upper_bound {
            // handle upper bound
            // should rotate player towards down, and push them down as well
            let current_pointing_direction = transform.rotation * Vec3::Y;
            if current_pointing_direction.x < 0.0 {
                // rotate to the left
                transform.rotation *= Quat::from_rotation_z(strength);
            } else if current_pointing_direction.x > 0.0 {
                // rotate to the right
                transform.rotation *= Quat::from_rotation_z(-strength);
            } else {
                // do nothing.
            }
            physics.velocity.y -= game.config.upper_repulsion_strength;
        } else if transform.translation.y < game.config.lower_bound {
            // handle lower bound
            // should rotate player towards up, and push them up as well
            let current_pointing_direction = transform.rotation * Vec3::Y;
            if current_pointing_direction.x < 0.0 {
                // rotate to the right
                transform.rotation *= Quat::from_rotation_z(-strength);
            } else if current_pointing_direction.x > 0.0 {
                // rotate to the left
                transform.rotation *= Quat::from_rotation_z(strength);
            } else {
                // do nothing.
            }
            physics.velocity.y += game.config.lower_repulsion_strength;
        } else {
            continue;
        }
    }
}
