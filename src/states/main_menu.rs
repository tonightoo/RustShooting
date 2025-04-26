use crate::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_menu)
            .add_systems(Update, menu_system.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Text::new("- Press Space to Start -"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(60.0),
            left: Val::Percent(20.0),
            ..default()
        },
    ));
}

fn menu_system(mut next_state: ResMut<NextState<GameState>>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
