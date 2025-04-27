use crate::GameState;
use crate::components::animation::*;
use crate::components::collider::*;
use crate::components::enemy::*;
use crate::components::wave::*;
use bevy::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), load_enemy_asset)
            .insert_resource(EnemySpawnTimer {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            })
            .add_systems(Update, spawn_enemy.run_if(in_state(GameState::Playing)))
            .add_systems(Update, enemy_movement.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_enemies);
    }
}

fn load_enemy_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Dino.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(30, 30), 2, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    let enemy_config = AnimationConfig::new(0, 1, 10, AnimationType::Loop);

    commands.insert_resource(EnemyAsset {
        texture,
        layout: layout_handle,
        anim_config: enemy_config,
    })
}

fn spawn_enemy(
    mut commands: Commands,
    mut interval: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    enemy_asset: Res<EnemyAsset>,
) {
    interval.timer.tick(time.delta());

    if !interval.timer.finished() {
        return;
    }

    let mut rng = rand::rng();
    let x = rng.random_range(-210.0..210.0);

    commands.spawn((
        Sprite::from_atlas_image(
            enemy_asset.texture.clone(),
            TextureAtlas {
                layout: enemy_asset.layout.clone(),
                index: enemy_asset.anim_config.first_sprite_index,
            },
        ),
        enemy_asset.anim_config.clone(),
        //Sprite {
        //    color: Color::srgb(1.0, 0.5, 0.0),
        //    custom_size: Some(Vec2::new(30.0, 30.0)),
        //    ..default()
        //},
        Transform::from_xyz(x, 340.0, 0.0),
        Collider {
            shape: ColliderShape::Rectangle {
                size: Vec2::new(30.0, 30.0),
            },
            tag: ColliderTag::Enemy,
        },
        Enemy,
    ));
}

fn enemy_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Enemy>>,
    time: Res<Time>,
    waves: Res<Waves>,
) {
    let speed: f32 = waves.waves[waves.current_wave].enemy_speed;

    for (entity, mut transform) in &mut query {
        transform.translation.y -= 1.0 * speed * time.delta_secs();

        if transform.translation.y <= -380.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn cleanup_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
