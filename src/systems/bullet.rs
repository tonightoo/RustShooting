use crate::GameState;
use crate::components::bullet::*;
use crate::components::collider::*;
use crate::components::player::Player;
use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletCooldown {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        })
        .add_systems(Update, bullet_spawn.run_if(in_state(GameState::Playing)))
        .add_systems(Update, bullet_movement.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_bullets);
    }
}

fn bullet_spawn(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    mut cooldown: ResMut<BulletCooldown>,
    time: Res<Time>,
) {
    cooldown.timer.tick(time.delta());
    if !keyboard.pressed(KeyCode::Space) {
        return;
    }

    if !cooldown.timer.finished() {
        return;
    }

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
            Bullet,
        ));
    }
}

fn bullet_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Bullet>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 600.0;
    for (entity, mut transform) in &mut query {
        transform.translation.y += SPEED * time.delta_secs();

        if transform.translation.y >= 360.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn cleanup_bullets(mut commands: Commands, query: Query<Entity, With<Bullet>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
