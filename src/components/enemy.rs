use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
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
