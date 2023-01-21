use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct Physics {
    pub velocity: Vec3,
    pub friction: f32,
    pub gravity: Vec3,
}

// TODO: implement Rotation component that syncs to transform and quat

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
        // counter += 1;
        // last_transform = Some(transform.clone().translation);
    }
    // if debug_timer.just_finished() {
    //     println!(
    //         "processed linear physics for {} entities. last translation was {:?}",
    //         counter, last_transform
    //     );
    // }
}
