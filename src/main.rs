//! Animates a sprite in response to a keyboard event.
//!
//! See `sprite_sheet.rs` for an example where the sprite animation loops indefinitely.

use std::time::Duration;

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup) // display sprite + txt
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
    for ev in evr_kbd.read() {
        match ev.state {
            ButtonState::Pressed => {
                match ev.key_code {
                    KeyCode::ArrowUp => { sprite.moving = true; sprite.direction = Direction::Up }
                    KeyCode::ArrowDown => { sprite.moving = true; sprite.direction = Direction::Down }
                    KeyCode::ArrowLeft => { sprite.moving = true; sprite.direction = Direction::Left }
                    KeyCode::ArrowRight => { sprite.moving = true; sprite.direction = Direction::Right }
                    _ => {}
                }
            }
            ButtonState::Released => { sprite.moving = false; }
        }
    }
}
fn perform_camera_tracking<C: Component>(
    mut evr_kbd: EventReader<KeyboardInput>, // bind to keyboard input event
    mut query: Query<&mut PanOrbitConfig, With<C>> // bind to camera
) {
    let mut camera = query.single_mut();
    for ev in evr_kbd.read() {
        match ev.state {
            ButtonState::Pressed => {
                match ev.key_code {
                    KeyCode::ArrowUp => { camera.moving = true; camera.direction = Direction::Up }
                    KeyCode::ArrowDown => { camera.moving = true; camera.direction = Direction::Down }
                    KeyCode::ArrowLeft => { camera.moving = true; camera.direction = Direction::Left }
                    KeyCode::ArrowRight => { camera.moving = true; camera.direction = Direction::Right }
                    _ => {}
                }
            }
            ButtonState::Released => { camera.moving = false; }
        }
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas, &mut Transform)>,
) {
    for (mut config, mut atlas, mut transform) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        if config.moving {
            // If it has been displayed for the user-defined amount of time (fps)...
            match config.direction {
                Direction::Up => { config.y += 150. * time.delta_seconds() }
                Direction::Down => { config.y -= 150. * time.delta_seconds() }
                Direction::Left => { config.x -= 150. * time.delta_seconds() }
                Direction::Right => { config.x += 150. * time.delta_seconds() }
            }
            transform.translation.x = config.x;
            transform.translation.y = config.y;

            if config.frame_timer.just_finished() {
                if atlas.index == config.last_sprite_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_sprite_index + 1;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    // ...and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        } else { atlas.index = config.first_sprite_index; }
    }
}

fn camera_tracking(
    time: Res<Time>,
    mut query: Query<(&mut PanOrbitConfig, &mut Transform)>,
) {
    for (mut config, mut transform) in &mut query {
        if config.moving {
            match config.direction {
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

#[derive(Component)]
struct AnimationConfig {
    moving: bool,
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
    x: f32,
    y: f32,
    direction: Direction,
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            moving: false,
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            x: 0.,
            y: 0.,
            direction: Direction::Right
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
}

#[derive(Component)]
struct PlayerSprite;

#[derive(Component)]
struct SceneCamera;

// Bundle to spawn our custom camera easily
#[derive(Bundle, Default)]
struct PanOrbitCameraBundle {
    camera: Camera2dBundle,
    config: PanOrbitConfig,
}

// The internal state of the pan-orbit controller
#[derive(Component)]
struct PanOrbitConfig {
    moving: bool,
    x: f32,
    y: f32,
    direction: Direction
}

impl Default for PanOrbitConfig {
    fn default() -> Self {
        PanOrbitConfig {
            moving: false,
            x: 0.,
            y: 0.,
            direction: Direction::Right
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanOrbitAction {
    Pan,
    Orbit,
    Zoom,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    
    // load the sprite sheet using the `AssetServer`
    let texture = asset_server.load("textures/gabe-idle-run.png");
    
    // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    
    // the first sprite runs at 10 FPS
    let animation_character = AnimationConfig::new(0, 6, 6);
    
    // spawn random sprites
    commands.spawn(( SpriteBundle { transform: Transform::from_xyz(100., 100., 0.), texture: texture.clone(), ..default() }, TextureAtlas { layout: texture_atlas_layout.clone(), index: 0 } ));
    commands.spawn(( SpriteBundle { transform: Transform::from_xyz(-100., -100., 0.), texture: texture.clone(), ..default() }, TextureAtlas { layout: texture_atlas_layout.clone(), index: 0 } ));
    
    // create the player sprite
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture: texture.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_character.first_sprite_index,
        },
        PlayerSprite,
        animation_character,
    ));

    let camera = PanOrbitCameraBundle::default();
    commands.spawn((
        camera,
        SceneCamera
    ));
}
