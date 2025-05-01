use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub is_player: bool,
    pub speed: f32,
}

#[derive(Resource)]
pub struct BulletCooldown {
    pub timer: Timer,
}
