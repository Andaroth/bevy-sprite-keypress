use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Component, Serialize, Deserialize)]
pub struct SpriteSet {
    pub set: String,
    pub size: u32,
    pub faces: u32,
}
