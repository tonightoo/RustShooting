use crate::GameState;
use bevy::prelude::*;

pub struct PlayingPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_playing)
            .add_systems(Update, playing_system.run_if(in_state(GameState::Playing)))
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
            .add_systems(Update, player_shoot.run_if(in_state(GameState::Playing)))
            .add_systems(Update, bullet_movement.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_playing);
    }
}

fn setup_playing(commands: Commands) {
    spawn_player(commands);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.7, 1.0),
            custom_size: Some(Vec2::new(30.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -300.0, 0.0),
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
) {
    if !keyboard.pressed(KeyCode::Space) {
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

fn playing_system(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    println!("Playing: Game is on!");
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::GameOver);
    }
}

fn cleanup_playing(mut commands: Commands, mut query: Query<Entity, With<Player>>) {
    let entity = query.single_mut();
    commands.entity(entity).despawn();
}
