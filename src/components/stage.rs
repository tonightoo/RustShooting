use crate::components::wave::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Stage {
    Stage1,
    Stage2,
    Stage3,
}

#[derive(Resource)]
pub struct StageSelection {
    pub stages: Vec<Stage>,
    pub current_index: usize,
}

#[derive(Component)]
pub struct StageSelectScreenTag;

#[derive(Debug, Clone)]
pub struct StageSetting {
    pub background_image: Handle<Image>,
    pub waves: Waves,
}

#[derive(Resource)]
pub struct StageDatabase {
    pub settings: HashMap<Stage, StageSetting>,
}
