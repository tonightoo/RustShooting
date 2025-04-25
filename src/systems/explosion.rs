use bevy::prelude::*;

use crate::GameState;
use crate::components::animation::AnimationConfig;
use crate::components::explosion::ExplosionAsset;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), load_explosion_asset);
    }
}

fn load_explosion_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("explosion.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(170, 196), 4, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    let explosion_config = AnimationConfig::new(0, 3, 10);

    commands.insert_resource(ExplosionAsset {
        texture,
        layout: layout_handle,
        anim_config: explosion_config,
    })
}

pub fn spawn_explosion(commands: &mut Commands, position: Vec3, explosion_asset: &ExplosionAsset) {
    commands.spawn((
        Sprite::from_atlas_image(
            explosion_asset.texture.clone(),
            TextureAtlas {
                layout: explosion_asset.layout.clone(),
                index: explosion_asset.anim_config.first_sprite_index,
            },
        ),
        Transform::from_translation(position),
        explosion_asset.anim_config.clone(),
    ));
}
