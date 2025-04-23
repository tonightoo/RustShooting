use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
