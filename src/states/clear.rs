use crate::GameState;
use crate::components::assets::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct ClearPlugin;

impl Plugin for ClearPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Clear), setup_game_over)
            .add_systems(OnEnter(GameState::Clear), start_bgm)
            .add_systems(Update, game_over_system.run_if(in_state(GameState::Clear)))
            .add_systems(OnExit(GameState::Clear), cleanup_game_over)
            .add_systems(OnExit(GameState::Clear), end_bgm);
    }
}

fn start_bgm(assets: Res<GameAssets>, audio: Res<bevy_kira_audio::prelude::Audio>) {
    audio
        .play(assets.clear_bgm.clone())
        .with_volume(0.2)
        .looped();
}

fn setup_game_over(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::BLACK));
}

fn game_over_system(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    //println!("Game Over: Press R to Restart");
    if keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
}

fn cleanup_game_over() {}

fn end_bgm(audio: Res<bevy_kira_audio::prelude::Audio>) {
    audio.stop();
}
