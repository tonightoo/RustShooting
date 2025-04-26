use crate::GameState;
use crate::components::collider::*;
use crate::components::player::Player;
use crate::components::player::PlayerAsset;
use crate::systems::sets::MySystemSet;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            load_player_asset.in_set(MySystemSet::LoadAssets),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            spawn_player.after(MySystemSet::LoadAssets),
        )
        .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_player);
    }
}

fn load_player_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("Rocket.png");
    commands.insert_resource(PlayerAsset { texture });
}

fn spawn_player(mut commands: Commands, player_asset: Res<PlayerAsset>) {
    commands.spawn((
        Sprite::from_image(player_asset.texture.clone()),
        //Sprite {
        //    color: Color::srgb(0.3, 0.7, 1.0),
        //    custom_size: Some(Vec2::new(30.0, 40.0)),
        //    ..default()
        //},
        Transform::from_xyz(0.0, -300.0, 0.0),
        Collider {
            shape: ColliderShape::Rectangle {
                size: Vec2::new(30.0, 40.0),
            },
            tag: ColliderTag::Player,
        },
        Player,
    ));
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

fn cleanup_player(mut commands: Commands, mut query: Query<Entity, With<Player>>) {
    let entity = query.single_mut();
    commands.entity(entity).despawn_recursive();
}
