use crate::GameState;
use crate::components::player::Player;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_player);
    }
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

fn cleanup_player(mut commands: Commands, mut query: Query<Entity, With<Player>>) {
    let entity = query.single_mut();
    commands.entity(entity).despawn_recursive();
}
