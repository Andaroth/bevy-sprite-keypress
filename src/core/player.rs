use crate::structs::base::BaseAnimation;
use crate::structs::sprites::PlayerSprite;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // create the player sprite
    let character_img = asset_server.load("textures/gabe-idle-run.png"); // load the sprite sheet using the `AssetServer`
    let animation_character = BaseAnimation::new(0, 6, 6); // the first sprite runs at 6 FPS
    let character_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(24),
        7,
        1,
        None,
        None,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture: character_img.clone(),
            ..default()
        },
        TextureAtlas {
            layout: character_layout.clone(),
            index: animation_character.first_sprite_index,
        },
        PlayerSprite,
        animation_character,
    ));
}
