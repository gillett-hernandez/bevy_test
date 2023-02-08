use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    enemy::Enemy,
    events::EnemyHit,
    misc::{CollisionRadius, HP},
};

#[derive(Component)]
pub struct Laser {
    pub damage: f32,
    pub hostile: bool,
    pub width: f32,
    pub max_dist: f32,
}

impl Laser {
    pub fn new(damage: f32, hostile: bool, width: f32, max_dist: f32) -> Self {
        assert!(width > 0.0);
        Laser {
            damage,
            hostile,
            width,
            max_dist,
        }
    }
}

pub fn enemy_laser_collision_system(
    mut enemies: Query<(Entity, &mut HP, &CollisionRadius, &Transform), With<Enemy>>,
    mut hit_events: EventWriter<EnemyHit>,
    lasers: Query<(&Laser, &GlobalTransform)>,
) {
    // no acceleration structure, complexity is O(n*m) where n is laser count and m is enemy count
    // in most cases, laser count will just be one, so there's no need to worry about complexity

    // in the future if multiplayer is implemented, we'll need enemy deaths to be attributed to player sources
    // to properly allocate scores

    for (laser, transform) in lasers.iter() {
        // let entity = commands.get_entity(entity)

        let (_, rotation, translation) = transform.to_scale_rotation_translation();
        let laser_origin = translation.truncate();
        // let laser_origin = transform.translation().truncate();
        // let comparison_vec = Vec3::new(1.0, 0.0, 0.0);
        // let angle
        // let cos = (transform.rotation * comparison_vec ).dot(comparison_vec);
        // let sin = .length_squared().sqrt();
        // info!("{:?}", axis);
        // let (dy, dx) = (angle + PI/2.0).sin_cos();
        let (_, angle) = rotation.to_axis_angle();
        // info!("{:?} {:?}", axis, angle);
        let (dy, dx) = (angle + PI / 2.0).sin_cos();
        let direction = Vec2::new(dx, dy);

        // need to determine if the laser overlaps with the enemies' hitbox
        // use circle hitboxes to begin with

        for (enemy_entity, mut hp, &enemy_radius, enemy_pos) in enemies.iter_mut() {
            let enemy_pos = enemy_pos.translation.truncate();

            let v = enemy_pos - laser_origin;
            let cos = v.normalize().dot(direction);
            if cos < 0.0 {
                // only allow laser damage in front of the laser
                continue;
            }
            let proj = v.project_onto(direction);
            let rej = v - proj;
            if rej.length_squared() < (laser.width + *enemy_radius).powi(2)
                && proj.length_squared() < laser.max_dist * laser.max_dist
            {
                hit_events.send(EnemyHit {
                    entity: enemy_entity,
                    damage: laser.damage,
                });
                hp.hp -= laser.damage;
            }
        }
    }
}
