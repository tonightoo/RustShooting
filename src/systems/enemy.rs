use crate::GameState;
use crate::components::enemy::*;
use bevy::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_enemy.run_if(in_state(GameState::Playing)))
        .add_systems(Update, enemy_movement.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_enemies);
    }
}

fn spawn_enemy(mut commands: Commands, mut interval: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    interval.timer.tick(time.delta());

    if !interval.timer.finished() {
        return;
    }

    let mut rng = rand::rng();
    let x = rng.random_range(-220.0..220.0);

    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.5, 0.0),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(x, 340.0, 0.0),
        Enemy,
    ));
}

fn enemy_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Enemy>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.0;

    for (entity, mut transform) in &mut query {
        transform.translation.y -= 1.0 * SPEED * time.delta_secs();

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
