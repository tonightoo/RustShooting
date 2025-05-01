use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum ItemType {
    RapidFire,
    PiercingShot,
    Heal,
}
