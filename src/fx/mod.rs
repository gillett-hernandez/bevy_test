use bevy::prelude::*;

use crate::{misc::HP, player::Player};

#[derive(Component)]
pub struct InnerHPCircle;
#[derive(Component)]
pub struct OuterHPCircle;

pub fn hp_visualizer_system(
    // commands: Commands,
    mut _inner_hp_circle: Query<&mut Transform, With<InnerHPCircle>>,
    mut outer_hp_circle: Query<&mut Visibility, With<OuterHPCircle>>,
    player: Query<&HP, (With<Player>, Changed<HP>)>,
) {
    for hp in player.iter() {
        // let mut inner_circle_transform = inner_hp_circle.get_single_mut().unwrap();
        let mut outer_circle_visibility = outer_hp_circle.get_single_mut().unwrap();

        // scale approaches 0 as hp approaches 0
        let scale = hp.hp / hp.max;
        if scale >= 0.9 {
            // hide sprite as hp is above threshold
            outer_circle_visibility.is_visible = false;
        } else {
            // actually display hp visual effect

            // TODO: fix this or rework to use a shader or texture
            // outer_circle_visibility.is_visible = true;
            // inner_circle_transform.scale = Vec3::new(scale, scale, 1.0);
        }
    }
}
