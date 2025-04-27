use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum ItemType {
    RapidFire,
    PiercingShot,
    Heal,
}

#[derive(Resource, Clone)]
pub struct ItemAssets {
    pub rapid_fire_texture: Handle<Image>,
    pub piercing_shot_texture: Handle<Image>,
    pub heal_texture: Handle<Image>,
}
