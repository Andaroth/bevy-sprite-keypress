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
        .add_systems(Update, execute_animations)
        .add_systems(Update, handle_keypress::<PlayerSprite>)
        .run();
}


#[derive(Component)]
struct AnimationConfig {
    playing: bool,
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

fn handle_keypress<S: Component>(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut query: Query<&mut AnimationConfig, With<S>>
) {
    let mut sprite = query.single_mut();
    for ev in evr_kbd.read() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({:?})", ev.key_code, ev.logical_key);
                sprite.playing = true;
                match ev.key_code {
                    KeyCode::ArrowUp => { sprite.direction = Direction::Up }
                    KeyCode::ArrowDown => { sprite.direction = Direction::Down }
                    KeyCode::ArrowLeft => { sprite.direction = Direction::Left }
                    KeyCode::ArrowRight => { sprite.direction = Direction::Right }
                    _ => {}
                }
            }
            ButtonState::Released => {
                println!("Key release: {:?} ({:?})", ev.key_code, ev.logical_key);
                sprite.playing = false;
            }
        }
    }
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            playing: false,
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

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas, &mut Transform)>,
) {
    for (mut config, mut atlas, mut transform) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        if config.playing {
            // If it has been displayed for the user-defined amount of time (fps)...
            if config.frame_timer.just_finished() {
                if atlas.index == config.last_sprite_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_sprite_index;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    match config.direction {
                        Direction::Up => { config.y += 1000. * time.delta_seconds(); }
                        Direction::Down => { config.y -= 1000. * time.delta_seconds(); }
                        Direction::Left => { config.x -= 1000. * time.delta_seconds(); }
                        Direction::Right => { config.x += 1000. * time.delta_seconds(); }
                    }
                    transform.translation.x = config.x;
                    transform.translation.y = config.y;
                    // ...and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

#[derive(Component)]
struct PlayerSprite;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());

    // load the sprite sheet using the `AssetServer`
    let texture = asset_server.load("textures/gabe-idle-run.png");

    // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // the first sprite runs at 10 FPS
    let animation_config_1 = AnimationConfig::new(1, 6, 10);

    // create the first sprite
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture: texture.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_config_1.first_sprite_index,
        },
        PlayerSprite,
        animation_config_1,
    ));
}
