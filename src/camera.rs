use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{gamestate::GameState, physics::Physics, player::Player};

pub fn camera_startup_system(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    if query.is_empty() {
        commands.spawn(Camera2dBundle::default());
    }
}

pub fn camera_system(
    _time: Res<Time>,
    // _game: Res<GameConfig>,
    // mut cam_and_player: ParamSet<(
    //     Query<&mut Transform, With<Camera>>,
    //     Query<(&Transform, &Physics), With<Player>>,
    // )>,
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<(&Transform, &Physics), With<Player>>,
) {
    // keep camera focused on the player, with some influence from how they're moving and where they're aiming.

    let (player_translation, player_velocity, player_rotation) = {
        let Ok((transform, physics)) = player.get_single() else {
            return;
        };
        (transform.translation, physics.velocity, transform.rotation)
    };

    let Ok(mut cam_transform) = camera.get_single_mut() else {
        return;
    };

    let velocity_len = player_velocity.length();

    let cam_z = cam_transform.translation.z;
    cam_transform.translation = player_translation
        + player_velocity.normalize() * 100.0 * (1.0 - (-velocity_len/1000.0).exp()) // push camera in velocity direction, clamped to some maximum value (to prevent the player from being off-screen)
        + player_rotation * Vec3::new(0.0, 1.0, 0.0) * 10.0; // push camera in aiming direction slightly.
    cam_transform.translation.z = cam_z;
}

pub fn is_state_ingame(state: Res<State<GameState>>) -> ShouldRun {
    (*state.current() == GameState::InGame).into()
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame).with_system(camera_startup_system),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_run_criteria(is_state_ingame)
                .with_system(camera_system),
        );
    }
}
