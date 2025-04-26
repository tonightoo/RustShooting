use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct BulletCooldown {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct BulletSound {
    pub sound: Handle<bevy_kira_audio::AudioSource>,
}
