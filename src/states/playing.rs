use crate::GameState;
use bevy::prelude::*;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_playing)
            .add_systems(Update, playing_system.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_playing);
    }
}

fn setup_playing(mut commands: Commands) {}

fn playing_system(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    println!("Playing: Game is on!");
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::GameOver);
    }
}

fn cleanup_playing() {}
