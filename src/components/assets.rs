use crate::components::animation::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub player_texture: Handle<Image>,
    pub dino_assets: AnimAsset,
    pub explosion_assets: AnimAsset,
    pub fill_heart_texture: Handle<Image>,
    pub empty_heart_texture: Handle<Image>,
    pub rapid_fire_texture: Handle<Image>,
    pub piercing_shot_texture: Handle<Image>,
    pub apple_texture: Handle<Image>,
    pub blue_egg_texture: Handle<Image>,
    pub yellow_egg_texture: Handle<Image>,
    pub ground_texture: Handle<Image>,
    pub ocean_texture: Handle<Image>,
    pub universe_texture: Handle<Image>,

    pub shoot_sound: Handle<bevy_kira_audio::AudioSource>,
    pub damage_sound: Handle<bevy_kira_audio::AudioSource>,
    pub explosion_sound: Handle<bevy_kira_audio::AudioSource>,
    pub playing_bgm: Handle<bevy_kira_audio::AudioSource>,
    pub clear_bgm: Handle<bevy_kira_audio::AudioSource>,
}

#[derive(Clone)]
pub struct AnimAsset {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub anim_config: AnimationConfig,
}

#[derive(Clone)]
pub enum AssetVisual {
    Static(Handle<Image>),
    Animated(AnimAsset),
}
