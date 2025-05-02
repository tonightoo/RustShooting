use crate::GameState;
use crate::components::animation::*;
use crate::components::assets::*;
use crate::systems::sets::MySystemSet;
use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Title),
            load_assets.in_set(MySystemSet::LoadAssets),
        );
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let game_assets: GameAssets = GameAssets {
        player_texture: asset_server.load("textures/rocket.png"),
        dino_assets: load_dino(&asset_server, &mut texture_atlas_layouts),
        explosion_assets: load_explosion(&asset_server, &mut texture_atlas_layouts),
        fill_heart_texture: asset_server.load("textures/fill_heart.png"),
        empty_heart_texture: asset_server.load("textures/empty_heart.png"),
        rapid_fire_texture: asset_server.load("textures/rapid_fire.png"),
        piercing_shot_texture: asset_server.load("textures/piercing_shot.png"),
        apple_texture: asset_server.load("textures/apple.png"),
        blue_egg_texture: asset_server.load("textures/blue_egg.png"),
        yellow_egg_texture: asset_server.load("textures/yellow_egg.png"),
        ground_texture: asset_server.load("textures/ground.png"),
        ocean_texture: asset_server.load("textures/ocean.png"),
        universe_texture: asset_server.load("textures/universe.png"),

        shoot_sound: asset_server.load("sounds/shoot.ogg"),
        damage_sound: asset_server.load("sounds/damage.ogg"),
        explosion_sound: asset_server.load("sounds/explosion.ogg"),
        playing_bgm: asset_server.load("sounds/jumpstart.ogg"),
        clear_bgm: asset_server.load("sounds/PixelPulse.ogg"),
    };

    commands.insert_resource(game_assets);
}

fn load_explosion(
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> AnimAsset {
    let texture = asset_server.load("textures/explosion.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(170, 196), 4, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    let explosion_config = AnimationConfig::new(0, 3, 10, AnimationType::Once);

    return AnimAsset {
        texture,
        layout: layout_handle.clone(),
        anim_config: explosion_config,
    };
}

fn load_dino(
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> AnimAsset {
    let texture = asset_server.load("textures/Dino.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(30, 30), 2, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    let enemy_config = AnimationConfig::new(0, 1, 10, AnimationType::Loop);

    return AnimAsset {
        texture,
        layout: layout_handle.clone(),
        anim_config: enemy_config,
    };
}
