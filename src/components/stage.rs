use crate::components::wave::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum Stage {
    Stage1,
    Stage2,
    Stage3,
}

#[derive(Component)]
pub struct StageSelectScreenTag;

#[derive(Debug, Clone)]
pub struct StageSetting {
    pub name: String,
    pub background_image: Handle<Image>,
    pub waves: Vec<Wave>,
    pub current_index: usize,
}

#[derive(Resource)]
pub struct StageDatabase {
    pub settings: Vec<StageSetting>,
    pub current_index: usize,
}

#[derive(Component)]
pub struct BackgroundImage;
