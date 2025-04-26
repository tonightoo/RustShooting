use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerAsset {
    pub texture: Handle<Image>,
}
