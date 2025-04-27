use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct Wave {
    pub number: u32,
    pub defeated_count: u32,
    pub target_count: u32,
    pub enemy_speed: f32,
    pub spawn_interval: f32,
}

#[derive(Resource)]
pub struct Waves {
    pub waves: Vec<Wave>,
    pub current_wave: usize,
}

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct ClearText;
