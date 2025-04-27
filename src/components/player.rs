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
pub struct PlayerAsset {
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct HeartAsset {
    pub fill_texture: Handle<Image>,
    pub empty_texture: Handle<Image>,
}

#[derive(Resource)]
pub struct DeadTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct DamageSound {
    pub sound: Handle<bevy_kira_audio::AudioSource>,
}
