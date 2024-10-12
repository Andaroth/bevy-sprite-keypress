
use std::time::Duration;
use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationConfig {
    pub moving: bool,
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
    pub x: f32, 
    pub y: f32,
    pub direction: Direction,
}

#[derive(Component)]
pub enum Direction { Up, Down, Left, Right }

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            moving: false,
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            x: 0., y: 0.,
            direction: Direction::Right
        }
    }
    pub fn timer_from_fps(fps: u8) -> Timer { Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating) }
}

#[derive(Component)]
pub struct PlayerSprite;

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
    pub direction: Direction
}

impl Default for PanOrbitConfig {
    fn default() -> Self {
        PanOrbitConfig {
            moving: false,
            x: 0., y: 0.,
            direction: Direction::Right
        }
    }
}