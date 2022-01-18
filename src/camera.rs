use bevy::prelude::*;

use crate::{gamestate::{Game, GameState}, physics::Physics, player::Player};

pub fn camera_startup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn camera_system(
    _time: Res<Time>,
    _game: Res<Game>,
    mut cam_and_player: QuerySet<(
        QueryState<&mut Transform, With<Camera>>,
        QueryState<(&Transform, &Physics), With<Player>>,
    )>,
) {
    // keep camera focused on the player, with some influence from how they're moving and where they're aiming.
    let (player_translation, player_velocity, player_rotation) = {
        let (temp_transform, temp_physics) = cam_and_player.q1().single();
        (
            temp_transform.translation,
            temp_physics.velocity,
            temp_transform.rotation, //.angle_between(Quat::IDENTITY),
        )
    };

    let mut q0 = cam_and_player.q0();
    let mut transform = q0.single_mut();

    let velocity_len = player_velocity.length();

    transform.translation = player_translation
        + player_velocity.normalize() * 100.0 * (1.0 - (-velocity_len/100.0).exp()) // push camera in velocity direction, clamped to some maximum value (to prevent the player from being off-screen)
        + player_rotation * Vec3::new(0.0, 1.0, 0.0) * 10.0; // push camera in aiming direction slightly.
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame).with_system(camera_startup_system),
        )
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(camera_system));
    }
}
