use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub score: u32,
}

#[derive(Component)]
pub struct ScoreText;
