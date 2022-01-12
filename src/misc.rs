use std::time::Duration;

use bevy::prelude::*;

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
