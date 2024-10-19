use bevy::prelude::*;
use bevy::asset::{io::Reader, AssetLoader, LoadContext, Asset};
use bevy::reflect::TypePath;
use serde::{Serialize, Deserialize};

use super::asset_loader::AssetLoaderError;
use crate::structs::scene::GameMap;

#[derive(Resource, Default)]
pub struct State {
    pub config: Handle<Config>,
    pub gamemap: Handle<GameMap>
}

#[derive(Debug, Asset, Default, TypePath, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub start_map: String,
    pub start_x: u32,
    pub start_y: u32
}

// impl AssetLoader for Config {
//     type Asset = Config;
//     type Settings = ();
//     type Error = AssetLoaderError;
//     async fn load<'a>(
//         &'a self,
//         _reader: &'a mut Reader<'_>,
//         _settings: &'a (),
//         _load_context: &'a mut LoadContext<'_>,
//     ) -> Result<Self::Asset, Self::Error> {
//         let name = "".to_string();
//         let start_map = "".to_string();
//         let start_x = 0;
//         let start_y = 0;

//         Ok(Config { name, start_map, start_x, start_y })
//     }
// }