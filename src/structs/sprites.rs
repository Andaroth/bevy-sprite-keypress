use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Component, Serialize, Deserialize)]
pub struct SpriteSet {
  pub set: String,
  pub size: u32,
  pub faces: u32
}