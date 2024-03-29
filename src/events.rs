// use std::marker::PhantomData;

use bevy::prelude::*;

use crate::mods::guns::WeaponType;

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

#[derive(Event)]
pub struct WeaponFired {
    pub entity: Entity, // the entity that fired the bullet
    pub entity_velocity: Vec3,
    pub hostile: bool,
    pub weapon_type: WeaponType,
}
impl WeaponFired {
    pub fn new(entity: Entity, entity_velocity: Vec3, hostile: bool, gun_type: WeaponType) -> Self {
        WeaponFired {
            entity,
            entity_velocity,
            hostile,
            weapon_type: gun_type,
        }
    }
}

#[derive(Default, Event)]
pub struct PlayerDeath;

#[derive(Event)]
pub struct EnemyDeath {
    pub entity: Entity,
    pub score: usize,
    pub heat: f32,
}

#[derive(Default, Event)]
pub struct PlayerHit {}

#[derive(Event)]
pub struct EnemyHit {
    pub entity: Entity,
    pub damage: f32,
}

pub struct EventsPlugin;
impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponFired>()
            .add_event::<PlayerHit>()
            .add_event::<EnemyHit>()
            .add_event::<PlayerDeath>()
            .add_event::<EnemyDeath>();
    }
}
