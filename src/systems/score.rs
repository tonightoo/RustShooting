use crate::GameState;
use crate::components::score::Score;
use crate::components::score::ScoreText;
use crate::systems::sets::MySystemSet;
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            load_score.in_set(MySystemSet::LoadAssets),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            spawn_score.after(MySystemSet::LoadAssets),
        )
        .add_systems(Update, update_score.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), despawn_score);
    }
}

fn load_score(mut commands: Commands) {
    commands.insert_resource(Score { score: 0 })
}

fn spawn_score(mut commands: Commands) {
    commands
        .spawn((Text::new(format!("Score: ")), ScoreText))
        .with_child((TextSpan::default(), ScoreText));
}

fn update_score(mut query: Query<&mut TextSpan, With<ScoreText>>, score: Res<Score>) {
    for mut span in &mut query {
        **span = format!("{}", score.score);
    }
}

fn despawn_score(mut commands: Commands, mut query: Query<Entity, With<ScoreText>>) {
    for entity in &mut query {
        commands.entity(entity).despawn_recursive();
    }
}
