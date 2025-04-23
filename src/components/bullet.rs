use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct BulletCooldown {
    pub timer: Timer,
}
