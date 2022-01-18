// use std::marker::PhantomData;

use bevy::prelude::*;

use crate::gun_collection::GunType;

// pub struct BulletFired<T> {
//     // where T is the type of bullet fired
//     pub entity: Entity,
//     pub hostile: bool,
//     _phantom: PhantomData<*const T>,
//     // location: Vec3,
//     // velocity: Vec3,
// }

// impl<T> BulletFired<T> {
//     pub fn new(entity: Entity, hostile: bool) -> Self {
//         BulletFired {
//             entity,
//             hostile,
//             _phantom: PhantomData,
//         }
//     }
// }

// unsafe impl<T> Send for BulletFired<T> {}
// unsafe impl<T> Sync for BulletFired<T> {}

pub struct BulletFired {
    pub entity: Entity, // the entity that fired the bullet
    pub hostile: bool,
    pub bullet_type: GunType,
}
impl BulletFired {
    pub fn new(entity: Entity, hostile: bool, bullet_type: GunType) -> Self {
        BulletFired {
            entity,
            hostile,
            bullet_type,
        }
    }
}

pub struct PlayerDeath;
