// use bevy::prelude::*;

// use crate::ai::{AIType, AI};

// use super::Enemy;

// basic enemy type. uses basic ai
// needs sprite
// needs to handle collision of non-hostile bullets with self

// #[derive(Bundle)]
// pub struct BasicEnemyBundle {
//     global_transform: GlobalTransform,
//     transform: Transform,
//     #[bundle]
//     sprite: SpriteBundle,
//     ai: AI,
//     enemy_type: Enemy,
// }
// impl BasicEnemyBundle {
//     pub fn new(position: Vec3, asset_server: Res<AssetServer>) -> Self {
//         BasicEnemyBundle {
//             global_transform: GlobalTransform::identity(),
//             transform: Transform {
//                 translation: position,
//                 ..Default::default()
//             },
//             ai: AI::new(AIType::Basic),
//             sprite: SpriteBundle {
//                 texture: asset_server.get_handle("images/enemy/basic_enemy.png"),
//                 ..Default::default()
//             },
//             enemy_type: Enemy { hp: 100.0 },
//         }
//     }
// }
