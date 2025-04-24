use crate::components::animation::AnimationConfig;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct ExplosionAsset {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub anim_config: AnimationConfig,
}
