use bevy::prelude::*;
mod components;
mod states;
mod systems;
use states::*;
use states::{GameOverPlugin, PlayingPlugin};

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My Vertical Shooter".into(),
                resolution: (480., 720.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(MainMenuPlugin)
        .add_plugins(PlayingPlugin)
        .add_plugins(GameOverPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
