use bevy::{platform::collections::HashMap, prelude::*};

use crate::player::Player;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct TextureAtlasHashMap(HashMap<String, Handle<TextureAtlasLayout>>);

// normal sprite

// animated sprite

#[derive(Component)]

pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

pub fn animate_player_sprite(
    mut query: Query<(&mut Sprite, &AnimationIndices), Without<Player>>,
    player: Query<Entity, With<Player>>,
) -> Result<(), BevyError> {
    let player = player.single()?;

    let (mut sprite, indices) = query.get_mut(player)?;
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = if atlas.index == indices.last {
            indices.first
        } else {
            atlas.index + 1
        };
    }

    Ok(())
}
