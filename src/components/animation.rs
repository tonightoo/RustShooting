use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Clone)]
pub struct AnimationConfig {
    pub timer: Timer,
    pub fps: u8,
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub animation_type: AnimationType,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8, aninm_type: AnimationType) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            timer: Self::timer_from_fps(fps),
            animation_type: aninm_type,
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / (fps as f32)),
            TimerMode::Repeating,
        )
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum AnimationType {
    Loop,
    Once,
}
