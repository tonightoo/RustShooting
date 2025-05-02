use crate::GameState;
use crate::components::assets::*;
use crate::components::bullet::Bullet;
use crate::components::collider::*;
use crate::components::enemy::*;
use crate::components::player::Player;
use crate::components::stage::StageDatabase;
use bevy::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), initialize_spawn_timer)
            .add_systems(Update, spawn_enemy.run_if(in_state(GameState::Playing)))
            .add_systems(Update, enemy_movement.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                enemy_fire_system.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_enemies);
    }
}

fn initialize_spawn_timer(mut commands: Commands, stage_db: Res<StageDatabase>) {
    let stage_index = stage_db.current_index;
    let wave_index = stage_db.settings[stage_index].current_index;
    let spawn_interval = stage_db.settings[stage_index].waves[wave_index].spawn_interval;

    commands.insert_resource(EnemySpawnTimer {
        timer: Timer::from_seconds(spawn_interval, TimerMode::Repeating),
    });
}

fn spawn_enemy(
    mut commands: Commands,
    mut interval: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    assets: Res<GameAssets>,
    query: Query<Entity, With<Player>>,
) {
    interval.timer.tick(time.delta());

    if !interval.timer.finished() {
        return;
    }

    if let Ok(player) = query.single() {
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
            assets.dino_assets.texture.clone(),
            TextureAtlas {
                layout: assets.dino_assets.layout.clone(),
                index: assets.dino_assets.anim_config.first_sprite_index,
            },
        ),
        assets.dino_assets.anim_config.clone(),
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
    assets: Res<GameAssets>,
) {
    for (transform, mut timer) in query.iter_mut() {
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            commands.spawn((
                Sprite::from_image(assets.blue_egg_texture.clone()),
                //Sprite {
                //    color: Color::srgb(0.5, 0.0, 0.5),
                //    custom_size: Some(Vec2::new(3.0, 3.0)),
                //    ..default()
                //},
                Transform {
                    translation: transform.translation,
                    ..default()
                },
                Collider {
                    shape: ColliderShape::Rectangle {
                        size: Vec2::new(30.0, 30.0),
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
    stage_db: Res<StageDatabase>,
    //waves: Res<Waves>,
) {
    //let speed: f32 = waves.waves[waves.current_wave].enemy_speed;
    let stage_index = stage_db.current_index;
    let wave_index = stage_db.settings[stage_index].current_index;

    let speed: f32 = stage_db.settings[stage_index].waves[wave_index].enemy_speed;

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
                if let Ok(player) = player_query.single() {
                    let direction = (player.translation - transform.translation).normalize();
                    transform.translation += direction * speed * time.delta_secs();
                }
            }
        }

        if transform.translation.y <= -380.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn cleanup_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
