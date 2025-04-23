use crate::GameState;
use crate::systems::bullet::BulletPlugin;
use crate::systems::collision::collision_system;
use crate::systems::enemy::EnemyPlugin;
use crate::systems::player::PlayerPlugin;
use bevy::prelude::*;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(BulletPlugin)
            .add_systems(
                Update,
                collision_system.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, playing_system.run_if(in_state(GameState::Playing)));
    }
}

fn playing_system(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    println!("Playing: Game is on!");
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::GameOver);
    }
}
