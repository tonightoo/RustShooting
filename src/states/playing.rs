use crate::GameState;
use crate::systems::animation::AnimationPlugin;
use crate::systems::bullet::BulletPlugin;
use crate::systems::collision::CollisionPlugin;
use crate::systems::enemy::EnemyPlugin;
use crate::systems::player::PlayerPlugin;
use crate::systems::score::ScorePlugin;
use bevy::prelude::*;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), initialize_background)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(BulletPlugin)
            .add_plugins(AnimationPlugin)
            .add_plugins(CollisionPlugin)
            .add_plugins(ScorePlugin)
            .add_systems(Update, playing_system.run_if(in_state(GameState::Playing)));
    }
}

fn initialize_background(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::srgb(0.7, 0.44, 0.25)));
}

fn playing_system(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    //println!("Playing: Game is on!");
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::GameOver);
    }
}
