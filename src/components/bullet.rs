use bevy::prelude::*;

use super::assets::AssetVisual;

#[derive(Component)]
pub struct Bullet {
    pub is_player: bool,
    pub speed: f32,
}

#[derive(Clone)]
pub struct BulletDefinition {
    pub is_player: bool,
    pub speed: f32,
    pub damage: u32,
    pub collider_size: Vec2,

    pub visual: AssetVisual,
}

#[derive(Resource)]
pub struct BulletCooldown {
    pub timer: Timer,
}
