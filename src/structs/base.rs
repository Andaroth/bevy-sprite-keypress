use std::time::Duration;
use bevy::prelude::*;

#[derive(Component)]
pub enum BaseDirection { Up, Down, Left, Right }

#[derive(Component)]
pub struct BaseAnimation {
    pub moving: bool,
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
    pub x: f32, 
    pub y: f32,
    pub direction: BaseDirection,
}

impl BaseAnimation {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            moving: false,
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            x: 0., y: 0.,
            direction: BaseDirection::Right
        }
    }
    pub fn timer_from_fps(fps: u8) -> Timer { Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating) }
}