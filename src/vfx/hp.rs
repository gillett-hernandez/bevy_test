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
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    // #[uniform(1)]
    // circle_size: f32,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

#[allow(unused_mut)]
pub fn hp_effect_system(
    // commands: Commands,
    // mut inner_hp_circle: Query<&mut Transform, With<InnerHPCircle>>,
    // mut outer_hp_circle: Query<&mut Visibility, With<OuterHPCircle>>,
    player: Query<&HP, (With<Player>, Changed<HP>)>,
) {
    // if outer_hp_circle.is_empty() || inner_hp_circle.is_empty() {
    //     // do nothing on first frame when inner and outer circles have not been set up
    //     return;
    // }
    // for hp in player.iter() {
    //     // let mut inner_circle_transform = inner_hp_circle.get_single_mut().unwrap();
    //     let mut outer_circle_visibility = outer_hp_circle.get_single_mut().unwrap();

    //     // scale approaches 0 as hp approaches 0
    //     let scale = hp.hp / hp.max;
    //     if scale >= 0.9 {
    //         // hide sprite as hp is above threshold
    //         outer_circle_visibility.is_visible = false;
    //     } else {
    //         // actually display hp visual effect

    //         // TODO: fix this or rework to use a shader or texture
    //         // outer_circle_visibility.is_visible = true;
    //         // inner_circle_transform.scale = Vec3::new(scale, scale, 1.0);
    //     }
    // }
}

pub fn hp_effect_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    query: Query<Entity, (With<Player>, Without<HpEffectMarker>)>,
    // common_sprites: Res<CommonSprites>,
) {
    for e in query.iter() {
        info!("adding hp effect material");
        commands
            .entity(e)
            .insert(HpEffectMarker)
            .add_children(|parent| {
                parent
                    .spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(
                            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(2000.0, 2000.0)))),
                        ),
                        transform: Transform::from_xyz(0.0, 0.0, 2.0),
                        material: materials.add(CustomMaterial {
                            color: Color::WHITE,
                            // circle_size: 1.0,
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
