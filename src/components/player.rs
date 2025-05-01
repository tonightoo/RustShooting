use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub max_hp: i32,
    pub hp: i32,
    pub invincible_timer: Timer,
    pub shoot_interval: f32,
    pub piercing: bool,
}

#[derive(Component)]
pub struct Heart;

#[derive(Resource)]
pub struct DeadTimer {
    pub timer: Timer,
}
