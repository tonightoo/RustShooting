use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct BulletCooldown {
    timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletCooldown {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        })
        .add_systems(OnEnter(GameState::Playing), spawn_player)
        .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
        .add_systems(Update, player_shoot.run_if(in_state(GameState::Playing)))
        .add_systems(Update, bullet_movement.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_player)
        .add_systems(OnExit(GameState::Playing), cleanup_bullets);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.7, 1.0),
            custom_size: Some(Vec2::new(30.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 360.0, 0.0),
        Player,
    ));
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.0;

    for mut transform in &mut query {
        let mut direction = 0.0;
        if keyboard.pressed(KeyCode::ArrowLeft)
            || keyboard.pressed(KeyCode::KeyA)
            || keyboard.pressed(KeyCode::KeyH)
        {
            direction -= 1.0;
        }

        if keyboard.pressed(KeyCode::ArrowRight)
            || keyboard.pressed(KeyCode::KeyD)
            || keyboard.pressed(KeyCode::KeyL)
        {
            direction += 1.0;
        }

        transform.translation.x += direction * SPEED * time.delta_secs();

        transform.translation.x = transform.translation.x.clamp(-220.0, 220.0);
    }
}

fn player_shoot(
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

fn cleanup_player(mut commands: Commands, mut query: Query<Entity, With<Player>>) {
    let entity = query.single_mut();
    commands.entity(entity).despawn_recursive();
}

fn cleanup_bullets(mut commands: Commands, query: Query<Entity, With<Bullet>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
