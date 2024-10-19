use bevy::prelude::*;

// use super::state::{State, Config};

use crate::structs::base::{BaseAnimation, BaseDirection};
// use crate::structs::scene::GameMap;
use crate::structs::scene::{PanOrbitCameraBundle, PanOrbitConfig, SceneCamera};

pub fn setup(
    // mut state: ResMut<State>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // config_asset: Res<Assets<Config>>,
    // gamemap_asset: Res<Assets<GameMap>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (img, size, rows, x, y) in [
        // list of assets
        ("textures/generic-rpg-tree01.png", 74, 1, 100., 100.),
        ("textures/generic-rpg-tree02.png", 74, 1, -100., -100.),
        ("textures/sensei.png", 24, 1, -80., -140.),
    ] {
        // generate
        let img = asset_server.load(img);
        let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(size),
            rows,
            1,
            None,
            None,
        ));
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.),
                texture: img.clone(),
                ..default()
            },
            TextureAtlas {
                layout: layout.clone(),
                index: 0,
            },
        ));
    }

    // init camera
    let camera = PanOrbitCameraBundle::default();
    commands.spawn((camera, SceneCamera));
}

pub fn run_animations(
    time: Res<Time>,
    mut query: Query<(&mut BaseAnimation, &mut TextureAtlas, &mut Transform)>,
) {
    for (mut config, mut atlas, mut transform) in &mut query {
        config.frame_timer.tick(time.delta()); // refresh rate
        if config.moving {
            match config.direction {
                // entity orientation determines acceleration
                BaseDirection::Up => config.y += 150. * time.delta_seconds(),
                BaseDirection::Down => config.y -= 150. * time.delta_seconds(),
                BaseDirection::Left => {
                    config.x -= 150. * time.delta_seconds();
                    transform.scale.x = -1.; // flip sprite face left
                }
                BaseDirection::Right => {
                    config.x += 150. * time.delta_seconds();
                    transform.scale.x = 1.; // flip sprite face right
                }
            }
            transform.translation.x = config.x;
            transform.translation.y = config.y;

            if config.frame_timer.just_finished() {
                // after tick
                if atlas.index == config.last_sprite_index {
                    // on anim end
                    atlas.index = config.first_sprite_index + 1; // skip still state
                } else {
                    // animate
                    atlas.index += 1;
                    config.frame_timer = BaseAnimation::timer_from_fps(config.fps);
                }
            }
        } else {
            atlas.index = config.first_sprite_index;
        } // make still
    }
}

pub fn track_camera(time: Res<Time>, mut query: Query<(&mut PanOrbitConfig, &mut Transform)>) {
    for (mut config, mut transform) in &mut query {
        if config.moving {
            match config.direction {
                // entity orientation determines acceleration
                BaseDirection::Up => config.y += 150. * time.delta_seconds(),
                BaseDirection::Down => config.y -= 150. * time.delta_seconds(),
                BaseDirection::Left => config.x -= 150. * time.delta_seconds(),
                BaseDirection::Right => config.x += 150. * time.delta_seconds(),
            }
            transform.translation.x = config.x;
            transform.translation.y = config.y;
        }
    }
}
