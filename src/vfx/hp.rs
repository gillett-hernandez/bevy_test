use bevy::prelude::*;

use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

use crate::{misc::HP, player::Player};

// #[derive(Component)]
// pub struct InnerHPCircle;

// #[derive(Component)]
// pub struct OuterHPCircle;

#[derive(Component)]
pub struct HpEffectMarker;

#[derive(Component)]
pub struct HpEffectMaterial(Handle<HpEffectMaterialInner>);

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct HpEffectMaterialInner {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    circle_size: f32,
}

impl Material2d for HpEffectMaterialInner {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

pub fn hp_effect_system(
    player: Query<(&HP, &Children), (With<Player>, Changed<HP>)>,
    hp_effect: Query<&HpEffectMaterial>,
    mut assets: ResMut<Assets<HpEffectMaterialInner>>,
) {
    info_once!("hp effect system online");
    for (hp, children) in player.iter() {
        let mut maybe_material_handle = None;
        for &child in children {
            maybe_material_handle = hp_effect.get(child).ok();
        }
        let Some(material_handle) = maybe_material_handle else {
            continue;
        };

        let Some(material) = assets.get_mut(&material_handle.0) else {
            continue;
        };

        // scale approaches 0 as hp approaches 0
        let scale = hp.hp / hp.max;
        if scale >= 0.9 {
            // hide sprite as hp is above threshold
            material.circle_size = 1.0;
        } else {
            // actually display hp visual effect
            material.circle_size = scale / 2.0;
        }
    }
}

pub fn hp_effect_setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<HpEffectMaterialInner>>,
    query: Query<Entity, (With<Player>, Without<HpEffectMarker>)>,
) {
    for e in query.iter() {
        info!("adding hp effect material");
        commands
            .entity(e)
            .insert(HpEffectMarker)
            .with_children(|spawner| {
                spawner.spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    Transform::from_xyz(0.0, 0.0, 2.0),
                    HpEffectMaterial(materials.add(HpEffectMaterialInner {
                        color: LinearRgba::WHITE,
                        circle_size: 0.5,
                    })),
                    Visibility::Visible,
                ));

                // let unwrapped = common_sprites.hp_circle.as_ref().unwrap();
                // parent
                //     .spawn(MaterialMesh2dBundle {
                //         mesh: unwrapped.inner_circle_mesh.clone(),
                //         material: unwrapped.inner_circle_material.clone(),
                //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                //         ..default()
                //     })
                //     .insert(InnerHPCircle);

                // parent
                //     .spawn(MaterialMesh2dBundle {
                //         mesh: unwrapped.outer_circle_mesh.clone(),
                //         material: unwrapped.outer_circle_material.clone(),
                //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                //         ..default()
                //     })
                //     .insert(OuterHPCircle);
            });
    }
}
