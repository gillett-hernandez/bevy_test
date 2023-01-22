use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct Physics {
    pub mass: f32,
    pub velocity: Vec3,
    pub friction: f32,
    pub gravity: Vec3,
}

pub fn linear_physics(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Physics)>,
    // debug_timer: Res<Timer>,
) {
    // let mut counter = 0;
    // let mut last_transform = None;
    for (_, mut transform, mut physics) in query.iter_mut() {
        let gravity = physics.gravity;
        let friction = physics.friction;

        physics.velocity += gravity;

        if true {
            // linear friction
            physics.velocity *= friction;
        } else {
            let velocity = physics.velocity;
            let mass = physics.mass;

            // force = mass * acceleration
            // acceleration = force / mass
            let acceleration = -(velocity.length_squared() * (1.0 - friction) / mass).min(1.0);
            physics.velocity += velocity.normalize() * acceleration * time.delta_seconds();
        }

        transform.translation += physics.velocity * time.delta_seconds();
    }
}
