use bevy::prelude::*;

use crate::{
    enemy::Enemy,
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
    pub fn new(damage: f32, hostile: bool, width: f32, max_dist: f32,) -> Self {
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
    mut enemies: Query<(&mut HP, &CollisionRadius, &Transform), With<Enemy>>,
    lasers: Query<(&Laser, &Transform)>,
) {
    // no acceleration structure, complexity is O(n*m) where n is laser count and m is enemy count
    // in most cases, laser count will just be one, so there's no need to worry about complexity

    // in the future if multiplayer is implemented, we'll need enemy deaths to be attributed to player sources
    // to properly allocate scores

    for (laser, transform) in lasers.iter() {
        // let entity = commands.get_entity(entity)

        // let transform
        let laser_origin = transform.translation.truncate();
        let (dy, dx) = transform.rotation.to_axis_angle().1.sin_cos();
        let direction = Vec2::new(dx, dy);

        // need to determine if the laser overlaps with the enemies' hitbox
        // use circle hitboxes to begin with

        for (mut hp, &enemy_radius, enemy_pos) in enemies.iter_mut() {
            let enemy_pos = enemy_pos.translation.truncate();

            let cos = (enemy_pos - laser_origin).dot(direction);
            if cos < 0.0 {
                // only allow laser damage in front of the laser
                continue;
            }
            let rej = (enemy_pos - laser_origin).reject_from(direction);
            if rej.length_squared() < (laser.width + *enemy_radius).powi(2) {
                hp.hp -= laser.damage;
            }
        }
    }
}
