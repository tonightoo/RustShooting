use crate::GameState;
use crate::components::assets::*;
use crate::components::collider::*;
use crate::components::player::*;
use crate::systems::sets::MySystemSet;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            spawn_player.after(MySystemSet::LoadAssets),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            spawn_hp.after(MySystemSet::LoadAssets),
        )
        .add_systems(Update, update_heart.run_if(in_state(GameState::Playing)))
        .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
        .add_systems(
            Update,
            player_invincible_timer_system.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            player_blink_system.run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_player)
        .add_systems(OnExit(GameState::Playing), cleanup_heart);
    }
}

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(assets.player_texture.clone()),
        Transform::from_xyz(0.0, -300.0, 0.0),
        Collider {
            shape: ColliderShape::Rectangle {
                size: Vec2::new(30.0, 40.0),
            },
            tag: ColliderTag::Player,
        },
        Player {
            max_hp: 3,
            hp: 3,
            invincible_timer: Timer::from_seconds(1.0, TimerMode::Once),
            shoot_interval: 0.2,
            piercing: false,
        },
    ));
}

fn player_invincible_timer_system(time: Res<Time>, mut query: Query<&mut Player>) {
    for mut player in &mut query {
        if player.invincible_timer.remaining_secs() > 0.0 {
            player.invincible_timer.tick(time.delta());
        }
    }
}

fn player_blink_system(mut query: Query<(&Player, &mut Sprite)>) {
    for (player, mut sprite) in &mut query {
        if player.is_invincible() {
            let time = player.invincible_timer.elapsed_secs();
            let blink_speed = 0.1;

            if (time / blink_speed) as i32 % 2 == 0 {
                sprite.color.set_alpha(1.0);
            } else {
                sprite.color.set_alpha(0.0);
            }
        } else {
            sprite.color.set_alpha(1.0);
        }
    }
}

fn spawn_hp(mut commands: Commands, assets: Res<GameAssets>, query: Query<&Player>) {
    if let Ok(player) = query.single() {
        for i in 0..player.max_hp {
            commands.spawn((
                Sprite::from_image(assets.fill_heart_texture.clone()),
                Transform::from_xyz((i as f32) * 60.0, 330.0, 0.0),
                Heart,
            ));
        }
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 500.0;

    for mut transform in &mut query {
        let mut x_direction = 0.0;
        let mut y_direction = 0.0;
        if keyboard.pressed(KeyCode::ArrowLeft)
            || keyboard.pressed(KeyCode::KeyA)
            || keyboard.pressed(KeyCode::KeyH)
        {
            x_direction -= 1.0;
        }

        if keyboard.pressed(KeyCode::ArrowRight)
            || keyboard.pressed(KeyCode::KeyD)
            || keyboard.pressed(KeyCode::KeyL)
        {
            x_direction += 1.0;
        }

        if keyboard.pressed(KeyCode::ArrowDown)
            || keyboard.pressed(KeyCode::KeyS)
            || keyboard.pressed(KeyCode::KeyJ)
        {
            y_direction -= 1.0;
        }

        if keyboard.pressed(KeyCode::ArrowUp)
            || keyboard.pressed(KeyCode::KeyW)
            || keyboard.pressed(KeyCode::KeyK)
        {
            y_direction += 1.0;
        }

        transform.translation.x += x_direction * SPEED * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(-225.0, 225.0);

        transform.translation.y += y_direction * SPEED * time.delta_secs();
        transform.translation.y = transform.translation.y.clamp(-340.0, 340.0);
    }
}

fn update_heart(
    mut commands: Commands,
    player_query: Query<&Player>,
    heart_query: Query<Entity, With<Heart>>,
    assets: Res<GameAssets>,
) {
    for entity in &heart_query {
        commands.entity(entity).despawn();
    }
    if let Ok(player) = player_query.single() {
        for i in 0..player.hp {
            commands.spawn((
                Sprite::from_image(assets.fill_heart_texture.clone()),
                Transform::from_xyz((i as f32) * 60.0, 330.0, 0.0),
                Heart,
            ));
        }

        for i in player.hp..player.max_hp {
            commands.spawn((
                Sprite::from_image(assets.empty_heart_texture.clone()),
                Transform::from_xyz((i as f32) * 60.0, 330.0, 0.0),
                Heart,
            ));
        }
    }
}

fn cleanup_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn cleanup_heart(mut commands: Commands, query: Query<Entity, With<Heart>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

impl Player {
    pub fn is_invincible(&self) -> bool {
        !self.invincible_timer.finished()
    }
}
