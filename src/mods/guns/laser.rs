use bevy::prelude::*;

use crate::{enemy::Enemy, gamestate::GameState, physics::Physics};

#[derive(Component)]
pub struct Laser {
    pub hostile: bool,
    pub width: f32,
}

impl Laser {
    pub fn new(hostile: bool, width: f32) -> Self {
        assert!(width > 0.0);
        Laser {
            hostile: false,
            width,
        }
    }
}

pub fn enemy_laser_collision_system(
    mut commands: Commands,
    mut query1: Query<(Entity, &mut Enemy, &Physics, &Transform)>,
    mut query2: Query<(Entity, &Laser, &Transform)>,
) {
    // no acceleration structure, complexity is O(n*m) where n is laser count and m is enemy count
    // in most cases, laser count will just be one, so there's no need to worry about complexity

    // in the future if multiplayer is implemented, we'll need enemy deaths to be attributed to player sources
    // to properly allocate scores

    for (_, laser, transform) in query2.iter() {
        // let entity = commands.get_entity(entity)

        // let transform
        let laser_origin = transform.translation;
        let (dy, dx) = transform.rotation.to_axis_angle().1.sin_cos();

        // need to determine if the laser overlaps with the enemies' hitbox
        // use circle hitboxes to begin with

        for (enemy_entity_id, enemy, enemy_phys, enemy_pos) in query1.iter_mut() {
            // enemy.
        }
    }
}

pub struct LaserCollisionPlugin;

impl Plugin for LaserCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                // .with_system(player_bullet_collision_system)
                .with_system(enemy_laser_collision_system),
        );
    }
}
