use bevy::prelude::*;

use crate::{enemy::Enemy, events::EnemyHit, physics::Physics};

use super::ParticleBundle;

pub fn enemy_hit_effect_system(
    mut commands: Commands,
    mut events: EventReader<EnemyHit>,
    query: Query<(&Transform, &Physics), With<Enemy>>,
    sprites: Res<Assets<Image>>,
) {
    // spawn short-lived particles
    if !events.is_empty() {
        println!("enemy hit effect system running");
    }
    for event in events.iter() {
        let num_particles = 1 + event.damage.floor() as usize;
        let Ok((
            transform,
            Physics {
                mass: _,
                velocity,
                friction: _,
                gravity: _,
            },
        )) = query.get(event.entity) else {
            continue;
        };
        for _ in 0..num_particles {
            commands
                .spawn(ParticleBundle::new(
                    &transform.clone(),
                    *velocity,
                    10.0,
                    1.0,
                ))
                .add_children(|p| {
                    p.spawn(SpriteBundle {
                        // TODO: replace with randomly chosen particle handle
                        texture: sprites.get_handle("images/bullet.png"),
                        transform: Transform::from_scale(Vec3::splat(0.1)),
                        ..default()
                    });
                });
        }
    }
}
