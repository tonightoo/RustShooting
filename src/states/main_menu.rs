use crate::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Title), setup_menu)
            .add_systems(Update, menu_system.run_if(in_state(GameState::Title)))
            .add_systems(OnExit(GameState::Title), cleanup_menu);
    }
}

fn setup_menu(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::BLACK));
    commands.spawn((
        Text::new("Defeat the Dinosaur\n\n\n\n- Press Space to Start -"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            //justify_content: JustifyContent::Center,
            //align_items: AlignItems::Center,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            //box_sizing: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            display: Display::Flex,
            //top: Val::Percent(60.0),
            //left: Val::Percent(20.0),
            ..default()
        },
    ));
}

fn menu_system(mut next_state: ResMut<NextState<GameState>>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::StageSelect);
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
