use bevy::prelude::*;
use systems::stage::StageSelectPlugin;
mod components;
mod states;
mod systems;
use states::*;
use states::{ClearPlugin, PlayingPlugin};

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Title,
    StageSelect,
    Playing,
    Clear,
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
        .add_plugins(bevy_kira_audio::prelude::AudioPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(MainMenuPlugin)
        .add_plugins(StageSelectPlugin)
        .add_plugins(PlayingPlugin)
        .add_plugins(ClearPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
