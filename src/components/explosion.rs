use crate::components::animation::AnimationConfig;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct ExplosionAsset {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub anim_config: AnimationConfig,
}

#[derive(Resource)]
pub struct ExplosionSound {
    pub sound: Handle<bevy_kira_audio::AudioSource>,
}

#[derive(Component, Clone)]
pub enum ExplosionTag {
    Player,
    Enemy,
}
