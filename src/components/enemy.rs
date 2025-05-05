use bevy::prelude::*;
use std::collections::HashMap;

use super::{assets::AssetVisual, bullet::BulletDefinition};

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Clone)]
pub struct EnemyDefinition {
    pub kind: EnemyKind,
    pub max_hp: u32,
    pub movement_pattern: EnemyMovePattern,
    pub collider_size: Vec2,
    pub fire_interval: f32,

    pub bullet: Option<BulletDefinition>,
    pub visual: AssetVisual,
}

#[derive(Resource)]
pub struct EnemyDatabase {
    pub defs: HashMap<EnemyKind, EnemyDefinition>,
}

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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum EnemyKind {
    DinoStraight,
    DinoZigzag,
    DinoHoming,
}
