use crate::GameState;
use crate::components::collider::*;
use crate::components::item::*;
use crate::components::player::*;
use crate::systems::sets::MySystemSet;
use bevy::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            load_items.in_set(MySystemSet::LoadAssets),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_items);
    }
}

pub fn load_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    let rapid_fire_texture = asset_server.load("rapid_fire.png");
    let piercing_shot_texture = asset_server.load("piercing_shot.png");
    let shield_texture = asset_server.load("shield.png");

    commands.insert_resource(ItemAssets {
        rapid_fire_texture,
        piercing_shot_texture,
        shield_texture,
    });
}

pub fn spawn_item(
    commands: &mut Commands,
    item_assets: &Res<ItemAssets>,
    item_type: ItemType,
    position: Vec3,
) {
    let asset = match item_type {
        ItemType::RapidFire => item_assets.rapid_fire_texture.clone(),
        ItemType::PiercingShot => item_assets.piercing_shot_texture.clone(),
        ItemType::Shield => item_assets.shield_texture.clone(),
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
        commands.entity(entity).despawn_recursive();
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
        ItemType::Shield => {
            player.hp += 1;
            if player.max_hp < player.hp {
                player.hp = player.max_hp;
            }
        }
    }
}
