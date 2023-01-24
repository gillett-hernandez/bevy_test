use bevy::prelude::*;

pub mod basic;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AIType {
    Basic,
}

#[derive(Component)]
pub struct AI {
    pub ai_type: AIType,
    // pub _should_fire_bullet: bool,
}

impl AI {
    pub fn new(ai_type: AIType) -> Self {
        AI {
            ai_type,
            // _should_fire_bullet: false,
        }
    }
    // pub fn should_fire_bullet(&self) -> bool {
    //     match self.ai_type {
    //         AIType::Basic => self._should_fire_bullet,
    //     }
    // }
}
