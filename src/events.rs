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

pub struct GunFired {
    pub entity: Entity, // the entity that fired the bullet
    pub hostile: bool,
    pub gun_type: GunType,
}
impl GunFired {
    pub fn new(entity: Entity, hostile: bool, gun_type: GunType) -> Self {
        GunFired {
            entity,
            hostile,
            gun_type,
        }
    }
}

pub struct PlayerDeath;

pub struct EventsPlugin;
impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GunFired>().add_event::<PlayerDeath>();
    }
}
