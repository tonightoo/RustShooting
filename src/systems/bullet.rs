use crate::GameState;
use crate::components::assets::*;
use crate::components::bullet::*;
use crate::components::collider::*;
use crate::components::player::Player;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletCooldown {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        })
        .add_systems(Update, update_cooldown.run_if(in_state(GameState::Playing)))
        .add_systems(Update, bullet_spawn.run_if(in_state(GameState::Playing)))
        .add_systems(Update, bullet_movement.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_bullets);
    }
}

fn update_cooldown(query: Query<&Player>, mut cooldown: ResMut<BulletCooldown>) {
    if let Ok(player) = query.get_single() {
        if Duration::from_secs_f32(player.shoot_interval.clone()) != cooldown.timer.duration() {
            cooldown.timer = Timer::from_seconds(player.shoot_interval, TimerMode::Repeating);
        }
    }
}

fn bullet_spawn(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    mut cooldown: ResMut<BulletCooldown>,
    time: Res<Time>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
    assets: Res<GameAssets>,
) {
    cooldown.timer.tick(time.delta());

    if keyboard.pressed(KeyCode::Space) && cooldown.timer.finished() {
        if let Ok(player_transform) = query.get_single() {
            let player_pos = player_transform.translation;
            commands.spawn((
                Sprite {
                    color: Color::srgb(1.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(3.0, 3.0)),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(player_pos.x, player_pos.y + 30.0, 0.0),
                    ..default()
                },
                Collider {
                    shape: ColliderShape::Rectangle {
                        size: Vec2::new(3.0, 3.0),
                    },
                    tag: ColliderTag::Bullet,
                },
                Bullet {
                    is_player: true,
                    speed: 600.0,
                },
            ));

            audio.play(assets.shoot_sound.clone()).with_volume(0.2);
            cooldown.timer.reset();
        }
    }
}

fn bullet_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Bullet), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut transform, bullet) in &mut query {
        if bullet.is_player {
            transform.translation.y += bullet.speed * time.delta_secs();
        } else {
            transform.translation.y -= bullet.speed * time.delta_secs();
        }

        if transform.translation.y >= 360.0 || transform.translation.y < -360.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn cleanup_bullets(mut commands: Commands, query: Query<Entity, With<Bullet>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
