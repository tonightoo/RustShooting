use crate::components::animation::AnimationConfig;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct EnemyAsset {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub anim_config: AnimationConfig,
}

#[derive(Component, Clone, Copy)]
pub enum EnemyMovePattern {
    Straight,
    Zigzag,
    Homing,
}

#[derive(Component)]
pub struct EnemyFireTimer {
    pub timer: Timer,
}
