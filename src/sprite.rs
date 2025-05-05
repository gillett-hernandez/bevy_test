use bevy::{platform::collections::HashMap, prelude::*};

use crate::player::Player;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct TextureAtlasHashMap(HashMap<String, (Handle<Image>, Handle<TextureAtlasLayout>)>);

// normal sprite

// animated sprite

#[derive(Component)]

pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}
