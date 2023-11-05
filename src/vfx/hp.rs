use bevy::prelude::*;

use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};

use crate::{misc::HP, player::Player};

// #[derive(Component)]
// pub struct InnerHPCircle;

// #[derive(Component)]
// pub struct OuterHPCircle;

#[derive(Component)]
pub struct HpEffectMarker;

// This is the struct that will be passed to your shader
#[derive(Asset, AsBindGroup, TypePath, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(1)]
    circle_size: f32,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

pub fn hp_effect_system(
    player: Query<(&HP, &Children), (With<Player>, With<HpEffectMarker>, Changed<HP>)>,
    hp_effect: Query<&Handle<CustomMaterial>>,
    mut assets: ResMut<Assets<CustomMaterial>>,
) {
    for (hp, children) in player.iter() {
        let mut maybe_material_handle = None;
        for &child in children {
            maybe_material_handle = hp_effect.get(child).ok();
        }
        let Some(material_handle) = maybe_material_handle else {
            continue;
        };

        let Some(material) = assets.get_mut(material_handle) else {
            continue;
        };

        // scale approaches 0 as hp approaches 0
        let scale = hp.hp / hp.max;
        if scale >= 0.7 {
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
    mut materials: ResMut<Assets<CustomMaterial>>,
    query: Query<Entity, (With<Player>, Without<HpEffectMarker>)>,
) {
    for e in query.iter() {
        info!("adding hp effect material");
        commands
            .entity(e)
            .insert(HpEffectMarker)
            .with_children(|spawner| {
                spawner
                    .spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(
                            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(3600.0, 3600.0)))),
                        ),
                        transform: Transform::from_xyz(0.0, 0.0, 2.0),
                        material: materials.add(CustomMaterial {
                            color: Color::WHITE,
                            circle_size: 0.5,
                        }),
                        ..default()
                    })
                    .insert(SpatialBundle::default());
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
