use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::render::camera::ClearColor;
use bevy::prelude::*;

mod structs;
use structs::{AnimationConfig,Direction, PlayerSprite, SceneCamera, PanOrbitCameraBundle, PanOrbitConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .insert_resource(ClearColor(Color::srgb(0.427, 0.969, 0.694))) // background color
        .add_systems(Startup, setup) // init sprites + camera
        .add_systems(Update, (
            execute_animations, // process
            camera_tracking, // process
            handle_character_move::<PlayerSprite>, // player move on keypress
            perform_camera_tracking::<SceneCamera>, // camera follows keypress
        ))
        .run();
}

fn handle_character_move<S: Component>(
    mut evr_kbd: EventReader<KeyboardInput>, // bind to keyboard input event
    mut query: Query<&mut AnimationConfig, With<S>> // bind to character
) {
    let mut sprite = query.single_mut();
    for ev in evr_kbd.read() { // each active keyboard input event
        match ev.state {
            ButtonState::Pressed => {
                sprite.moving = true; // free element
                match ev.key_code { // key binding
                    KeyCode::ArrowUp | KeyCode::KeyZ | KeyCode::KeyW => { sprite.direction = Direction::Up },
                    KeyCode::ArrowDown | KeyCode::KeyS => { sprite.direction = Direction::Down },
                    KeyCode::ArrowLeft | KeyCode::KeyQ | KeyCode::KeyA => { sprite.direction = Direction::Left },
                    KeyCode::ArrowRight | KeyCode::KeyD => { sprite.direction = Direction::Right },
                    _ => { sprite.moving = false; } // lock element
                }
            }
            ButtonState::Released => { sprite.moving = false; } // lock element
        }
    }
}
fn perform_camera_tracking<C: Component>(
    mut evr_kbd: EventReader<KeyboardInput>, // bind to keyboard input event
    mut query: Query<&mut PanOrbitConfig, With<C>> // bind to camera
) {
    let mut camera = query.single_mut();
    for ev in evr_kbd.read() { // each active keyboard input event
        match ev.state {
            ButtonState::Pressed => {
                camera.moving = true; // free element
                match ev.key_code { // key binding
                    KeyCode::ArrowUp | KeyCode::KeyZ | KeyCode::KeyW => { camera.direction = Direction::Up },
                    KeyCode::ArrowDown | KeyCode::KeyS => { camera.direction = Direction::Down },
                    KeyCode::ArrowLeft | KeyCode::KeyQ | KeyCode::KeyA => { camera.direction = Direction::Left },
                    KeyCode::ArrowRight | KeyCode::KeyD => { camera.direction = Direction::Right },
                    _ => { camera.moving = false; } // lock element
                }
            }
            ButtonState::Released => { camera.moving = false; } // lock element
        }
    }
}

fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut TextureAtlas, &mut Transform)>) {
    for (mut config, mut atlas, mut transform) in &mut query {
        config.frame_timer.tick(time.delta()); // refresh rate
        if config.moving {
            match config.direction { // entity orientation determines acceleration
                Direction::Up => { config.y += 150. * time.delta_seconds() }
                Direction::Down => { config.y -= 150. * time.delta_seconds() }
                Direction::Left => {
                    config.x -= 150. * time.delta_seconds();
                    transform.scale.x = -1.; // flip sprite face left
                }
                Direction::Right => {
                    config.x += 150. * time.delta_seconds();
                    transform.scale.x = 1.; // flip sprite face right
                }
            }
            transform.translation.x = config.x;
            transform.translation.y = config.y;

            if config.frame_timer.just_finished() { // after tick
                if atlas.index == config.last_sprite_index { // on anim end
                    atlas.index = config.first_sprite_index + 1; // skip still state
                } else { // animate
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        } else { atlas.index = config.first_sprite_index; } // make still
    }
}

fn camera_tracking(time: Res<Time>, mut query: Query<(&mut PanOrbitConfig, &mut Transform)>) {
    for (mut config, mut transform) in &mut query {
        if config.moving {
            match config.direction { // entity orientation determines acceleration
                Direction::Up => { config.y += 150. * time.delta_seconds() }
                Direction::Down => { config.y -= 150. * time.delta_seconds() }
                Direction::Left => { config.x -= 150. * time.delta_seconds() }
                Direction::Right => { config.x += 150. * time.delta_seconds() }
            }
            transform.translation.x = config.x;
            transform.translation.y = config.y;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (img, size, cols, x, y) in [ // list of assets
        ("textures/generic-rpg-tree01.png", 74, 1, 100., 100.),
        ("textures/generic-rpg-tree02.png", 74, 1, -100., -100.),
        ("textures/sensei.png", 24, 1, -80., -140.)
    ] { // generate
        let img = asset_server.load(img);
        let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(size), cols, 1, None, None));
        commands.spawn(( SpriteBundle { transform: Transform::from_xyz(x, y, 0.), texture: img.clone(), ..default() }, TextureAtlas { layout: layout.clone(), index: 0 } ));
    }
    // create the player sprite
    let character_img = asset_server.load("textures/gabe-idle-run.png"); // load the sprite sheet using the `AssetServer`
    let animation_character = AnimationConfig::new(0, 6, 6); // the first sprite runs at 6 FPS
    let character_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture:
            character_img.clone(), ..default()
        },
        TextureAtlas {
            layout: character_layout.clone(),
            index: animation_character.first_sprite_index
        },
        PlayerSprite,
        animation_character,
    ));
    // init camera
    let camera = PanOrbitCameraBundle::default();
    commands.spawn(( camera, SceneCamera ));
}
