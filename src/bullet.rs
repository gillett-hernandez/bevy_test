use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub hostile: bool,
}

// need to detect and handle collisions between hostile bullets and the player, and player bullets and enemies.

// pub fn bullet_system(query: Query<>) {}
