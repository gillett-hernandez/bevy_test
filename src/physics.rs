use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct Physics {
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
    for (e, mut transform, mut physics) in query.iter_mut() {
        let gravity = physics.gravity;
        let friction = physics.friction;
        physics.velocity += gravity;
        physics.velocity *= friction;

        transform.translation += physics.velocity * time.delta_seconds();
    }
}
