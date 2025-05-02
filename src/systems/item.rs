use crate::GameState;
use crate::components::assets::*;
use crate::components::collider::*;
use crate::components::item::*;
use crate::components::player::*;
use bevy::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), cleanup_items);
    }
}

pub fn spawn_item(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    item_type: ItemType,
    position: Vec3,
) {
    let asset = match item_type {
        ItemType::RapidFire => assets.rapid_fire_texture.clone(),
        ItemType::PiercingShot => assets.piercing_shot_texture.clone(),
        ItemType::Heal => assets.apple_texture.clone(),
    };

    commands.spawn((
        Sprite::from_image(asset.clone()),
        Transform::from_translation(position),
        Collider {
            shape: ColliderShape::Rectangle {
                size: Vec2::new(20.0, 20.0),
            },
            tag: ColliderTag::Item,
        },
        item_type,
    ));
}

pub fn cleanup_items(mut commands: Commands, query: Query<Entity, With<ItemType>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn apply_item_effect(player: &mut Player, item: ItemType) {
    match item {
        ItemType::RapidFire => {
            player.shoot_interval *= 0.5;
        }
        ItemType::PiercingShot => {
            player.piercing = true;
        }
        ItemType::Heal => {
            player.hp += 1;
            if player.max_hp < player.hp {
                player.hp = player.max_hp;
            }
        }
    }
}
