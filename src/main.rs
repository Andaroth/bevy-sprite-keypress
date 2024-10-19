use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::camera::ClearColor;

mod core;
use core::player::spawn_player;
use core::scene::{run_animations, setup, track_camera};
// use core::state::{State, Config};

mod structs;
use structs::base::{BaseAnimation, BaseDirection};
use structs::scene::{PanOrbitConfig, SceneCamera};
use structs::sprites::PlayerSprite;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        // .init_resource::<State>()
        // .init_asset_loader::<Config>()
        // .init_asset_loader::<GameMap>()
        .insert_resource(ClearColor(Color::srgb(0.427, 0.969, 0.694))) // background color
        .add_systems(Startup, (setup, spawn_player)) // init sprites + camera
        .add_systems(
            Update,
            (
                run_animations,                         // process
                track_camera,                           // process
                handle_character_move::<PlayerSprite>,  // player move on keypress
                perform_camera_tracking::<SceneCamera>, // camera follows keypress
            ),
        )
        .run();
}

fn handle_character_move<S: Component>(
    mut evr_kbd: EventReader<KeyboardInput>, // bind to keyboard input event
    mut query: Query<&mut BaseAnimation, With<S>>, // bind to character
) {
    let mut sprite = query.single_mut();
    for ev in evr_kbd.read() {
        // each active keyboard input event
        match ev.state {
            ButtonState::Pressed => {
                sprite.moving = true; // free element
                match ev.key_code {
                    // key binding
                    KeyCode::ArrowUp | KeyCode::KeyZ | KeyCode::KeyW => {
                        sprite.direction = BaseDirection::Up
                    }
                    KeyCode::ArrowDown | KeyCode::KeyS => sprite.direction = BaseDirection::Down,
                    KeyCode::ArrowLeft | KeyCode::KeyQ | KeyCode::KeyA => {
                        sprite.direction = BaseDirection::Left
                    }
                    KeyCode::ArrowRight | KeyCode::KeyD => sprite.direction = BaseDirection::Right,
                    _ => {
                        sprite.moving = false;
                    } // lock element
                }
            }
            ButtonState::Released => {
                sprite.moving = false;
            } // lock element
        }
    }
}
fn perform_camera_tracking<C: Component>(
    mut evr_kbd: EventReader<KeyboardInput>, // bind to keyboard input event
    mut query: Query<&mut PanOrbitConfig, With<C>>, // bind to camera
) {
    let mut camera = query.single_mut();
    for ev in evr_kbd.read() {
        // each active keyboard input event
        match ev.state {
            ButtonState::Pressed => {
                camera.moving = true; // free element
                match ev.key_code {
                    // key binding
                    KeyCode::ArrowUp | KeyCode::KeyZ | KeyCode::KeyW => {
                        camera.direction = BaseDirection::Up
                    }
                    KeyCode::ArrowDown | KeyCode::KeyS => camera.direction = BaseDirection::Down,
                    KeyCode::ArrowLeft | KeyCode::KeyQ | KeyCode::KeyA => {
                        camera.direction = BaseDirection::Left
                    }
                    KeyCode::ArrowRight | KeyCode::KeyD => camera.direction = BaseDirection::Right,
                    _ => {
                        camera.moving = false;
                    } // lock element
                }
            }
            ButtonState::Released => {
                camera.moving = false;
            } // lock element
        }
    }
}
