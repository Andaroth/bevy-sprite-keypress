use bevy::prelude::*;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use bevy::reflect::TypePath;
use serde::{Deserialize, Serialize};

use super::base::BaseDirection;
use super::sprites::SpriteSet;

use crate::core::asset_loader::AssetLoaderError;

#[derive(Component)]
pub struct SceneCamera;

// Bundle to spawn our custom camera easily
#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub camera: Camera2dBundle,
    pub config: PanOrbitConfig 
}

// The internal state of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitConfig {
    pub moving: bool,
    pub x: f32, 
    pub y: f32,
    pub direction: BaseDirection,
    // pub locked: bool
}

impl Default for PanOrbitConfig {
    fn default() -> Self {
        PanOrbitConfig {
            moving: false,
            x: 0., y: 0.,
            direction: BaseDirection::Right,
            // locked: false
        }
    }
}

#[derive(Component, Serialize, Deserialize)]
pub struct GameDecoration {
    pub sprite: SpriteSet,
    pub x: f32,
    pub y: f32
}

#[derive(Component, Serialize, Deserialize)]
pub struct GameEvent {
    pub sprite: SpriteSet,
    pub x: f32,
    pub y: f32
}

#[derive(Asset, Default, TypePath, Serialize, Deserialize)]
pub struct GameMap {
    pub id: u32,
    pub name: String,
    pub display_name: String,
    pub comment: String,
    pub note: String,
    pub decorations: Vec<GameDecoration>,
    pub common_events: Vec<GameEvent>
}

// impl AssetLoader for GameMap {
//     type Asset = GameMap;
//     type Settings = ();
//     type Error = AssetLoaderError;
//     async fn load<'a>(
//         &'a self,
//         _reader: &'a mut Reader<'_>,
//         _settings: &'a (),
//         _load_context: &'a mut LoadContext<'_>,
//     ) -> Result<Self::Asset, Self::Error> {
//         let id = 0;
//         let name = "".to_string();
//         let display_name = "".to_string();
//         let comment = "".to_string();
//         let note = "".to_string();
//         let decorations = Vec::<GameDecoration>::new();
//         let common_events = Vec::<GameEvent>::new();

//         Ok(GameMap { id, name, display_name, comment, note, decorations, common_events })
//     }
// }