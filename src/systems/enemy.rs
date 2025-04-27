use crate::GameState;
use crate::components::animation::*;
use crate::components::bullet::Bullet;
use crate::components::collider::*;
use crate::components::enemy::*;
use crate::components::player::Player;
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
            .add_systems(
                Update,
                enemy_fire_system.run_if(in_state(GameState::Playing)),
            )
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
    query: Query<Entity, With<Player>>,
) {
    interval.timer.tick(time.delta());

    if !interval.timer.finished() {
        return;
    }

    if let Ok(player) = query.get_single() {
    } else {
        return;
    }

    let mut rng = rand::rng();
    let x = rng.random_range(-210.0..210.0);

    let move_id = rng.random_range(0..3);
    let move_pattern = match move_id {
        0 => EnemyMovePattern::Straight,
        1 => EnemyMovePattern::Zigzag,
        2 => EnemyMovePattern::Homing,
        _ => EnemyMovePattern::Straight,
    };

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
        move_pattern,
        EnemyFireTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
    ));
}

fn enemy_fire_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut EnemyFireTimer)>,
) {
    for (transform, mut timer) in query.iter_mut() {
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.5, 0.0, 0.5),
                    custom_size: Some(Vec2::new(3.0, 3.0)),
                    ..default()
                },
                Transform {
                    translation: transform.translation,
                    ..default()
                },
                Collider {
                    shape: ColliderShape::Rectangle {
                        size: Vec2::new(3.0, 3.0),
                    },
                    tag: ColliderTag::EnemyBullet,
                },
                Bullet {
                    is_player: false,
                    speed: 400.0,
                },
            ));

            timer.timer.reset();
        }
    }
}

fn enemy_movement(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform, &EnemyMovePattern), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
    waves: Res<Waves>,
) {
    let speed: f32 = waves.waves[waves.current_wave].enemy_speed;

    for (entity, mut transform, pattern) in &mut enemy_query {
        match pattern {
            EnemyMovePattern::Straight => {
                transform.translation.y -= speed * time.delta_secs();
            }
            EnemyMovePattern::Zigzag => {
                transform.translation.y -= speed * time.delta_secs();
                transform.translation.x +=
                    (time.elapsed_secs() * 5.0).sin() * speed * time.delta_secs();
            }
            EnemyMovePattern::Homing => {
                if let Ok(player) = player_query.get_single() {
                    let direction = (player.translation - transform.translation).normalize();
                    transform.translation += direction * speed * time.delta_secs();
                }
            }
        }

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
