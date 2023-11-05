use bevy::prelude::*;
// use bevy::prelude::Camera2dBundle;
// use bevy::core_pipeline::prelude::Camera2dBundle;

use crate::{gamestate::GameState, misc::in_game_no_hitstun, physics::Physics, player::Player};

pub fn camera_startup_system(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    if query.is_empty() {
        commands.spawn(Camera2dBundle::default());
    }
}

pub fn camera_system(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player: Query<(&GlobalTransform, &Physics), (With<Player>, Without<Camera>)>,
) {
    // keep camera focused on the player, with some influence from how they're moving and where they're aiming.

    let Ok(mut cam_transform) = camera.get_single_mut() else {
        return;
    };

    let (player_translation, player_velocity, player_rotation) = {
        let Ok((transform, physics)) = player.get_single() else {
            return;
        };
        let (_, quaternion, translation) = transform.to_scale_rotation_translation();
        (translation, physics.velocity, quaternion)
    };

    let velocity_len = player_velocity.length();

    let cam_z = cam_transform.translation.z;

    // todo: zoom out when going fast?

    let speed_component =
        player_velocity.normalize() * 100.0 * (1.0 - (-velocity_len / 1000.0).exp()); // push camera in velocity direction, clamped to some maximum value (to prevent the player from being off-screen)
    let aim_component = player_rotation * Vec3::new(0.0, 1.0, 0.0) * 10.0; // push camera in aiming direction slightly.
    (*cam_transform).translation = player_translation + speed_component + aim_component;
    (*cam_transform).translation.z = cam_z;
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), camera_startup_system)
            .add_systems(PostUpdate, camera_system.run_if(in_game_no_hitstun));
    }
}
