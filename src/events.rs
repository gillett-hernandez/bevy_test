use std::marker::PhantomData;

use bevy::prelude::*;

pub struct BulletFired<T> {
    // where T is the type of bullet fired
    pub entity: Entity, // the entity that fired the bullet
    pub hostile: bool,
    _phantom: PhantomData<*const T>,
    // location: Vec3,
    // velocity: Vec3,
}

impl<T> BulletFired<T> {
    pub fn new(entity: Entity, hostile: bool) -> Self {
        BulletFired {
            entity,
            hostile,
            _phantom: PhantomData,
        }
    }
}

unsafe impl<T> Send for BulletFired<T> {}
unsafe impl<T> Sync for BulletFired<T> {}
