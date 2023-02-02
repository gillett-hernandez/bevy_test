use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{misc::HP, player::Player, sprite::CommonSprites};

#[derive(Component)]
pub struct InnerHPCircle;

#[derive(Component)]
pub struct OuterHPCircle;

#[allow(unused_mut)]
pub fn hp_effect_system(
    // commands: Commands,
    mut inner_hp_circle: Query<&mut Transform, With<InnerHPCircle>>,
    mut outer_hp_circle: Query<&mut Visibility, With<OuterHPCircle>>,
    player: Query<&HP, (With<Player>, Changed<HP>)>,
) {
    if outer_hp_circle.is_empty() || inner_hp_circle.is_empty() {
        // do nothing on first frame when inner and outer circles have not been set up
        return;
    }
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

pub fn hp_effect_setup_system(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
    common_sprites: Res<CommonSprites>,
) {
    for e in query.iter() {
        commands.entity(e).add_children(|parent| {
            let unwrapped = common_sprites.hp_circle.as_ref().unwrap();
            parent
                .spawn(MaterialMesh2dBundle {
                    mesh: unwrapped.inner_circle_mesh.clone(),
                    material: unwrapped.inner_circle_material.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..default()
                })
                .insert(InnerHPCircle);

            parent
                .spawn(MaterialMesh2dBundle {
                    mesh: unwrapped.outer_circle_mesh.clone(),
                    material: unwrapped.outer_circle_material.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..default()
                })
                .insert(OuterHPCircle);
        });
    }
}
