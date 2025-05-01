use bevy::prelude::*;

#[derive(Component, Clone)]
pub enum ExplosionTag {
    Player,
    Enemy,
}
