use bevy::prelude::*;

use crate::{
    ai::{AIType, AI},
    gun_collection::{GunData, GunType},
    misc::VerticallyBounded,
    physics::{Physics, Position},
};

pub mod basic;

#[derive(Component)]
pub struct Enemy {
    pub hp: f32,
}

// this mirrors the player hp system to an extent.

pub fn enemy_hp_system(mut commands: Commands, query: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in query.iter() {
        if enemy.hp <= 0.0 {
            // kill enemy if hp drops <= 0
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn add_basic_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn_bundle(BasicEnemyBundle::new(Vec3::ZERO, asset_server));
    commands
        .spawn_bundle((
            GlobalTransform::identity(),
            Transform {
                translation: Vec3::new(0.0, -100.0, 0.0),
                ..Default::default()
            },
            AI::new(AIType::Basic),
            Enemy { hp: 20.0 },
            Physics {
                velocity: Vec3::new(0.0, 0.0, 0.0),
                gravity: Vec3::new(0.0, -4.0, 0.0),
                friction: 0.99,
            },
            Position(Vec2::ZERO),
            VerticallyBounded {},
            GunType::MachineGun.data_from_type(asset_server.get_handle("bullet.png")),
        ))
        .with_children(|e| {
            // add sprite as child so that it's affected by the transform of the parent
            e.spawn_bundle(SpriteBundle {
                texture: asset_server.get_handle("enemy/basic_enemy.png"),
                transform: Transform {
                    scale: Vec3::splat(0.4),
                    translation: Vec3::new(0.0, 0.0, 1.0), // put on Z layer 1, above the background.
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
