use bevy::prelude::*;

use crate::components::assets::*;
use crate::components::explosion::*;

pub fn spawn_explosion(
    commands: &mut Commands,
    position: Vec3,
    assets: &Res<GameAssets>,
    tag: ExplosionTag,
) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.explosion_assets.texture.clone(),
            TextureAtlas {
                layout: assets.explosion_assets.layout.clone(),
                index: assets.explosion_assets.anim_config.first_sprite_index,
            },
        ),
        Transform::from_translation(position),
        assets.explosion_assets.anim_config.clone(),
        tag.clone(),
    ));
}
