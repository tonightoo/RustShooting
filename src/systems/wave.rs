use crate::GameState;
use crate::components::enemy::EnemySpawnTimer;
use crate::components::wave::*;
use bevy::prelude::*;

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), load_waves)
            .add_systems(Update, update_waves.run_if(in_state(GameState::Playing)));
    }
}

fn load_waves(
    mut commands: Commands,
    waves: Option<ResMut<Waves>>,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
) {
    if let Some(mut waves) = waves {
        waves.current_wave = 0;
        for wave in &mut waves.waves {
            wave.defeated_count = 0;
        }
        let interval = waves.waves[waves.current_wave].spawn_interval;
        enemy_timer.timer = Timer::from_seconds(interval, TimerMode::Repeating);
        return;
    }

    let new_waves = Waves {
        waves: vec![
            Wave {
                number: 0,
                defeated_count: 0,
                target_count: 10,
                enemy_speed: 200.0,
                spawn_interval: 2.0,
            },
            Wave {
                number: 1,
                defeated_count: 0,
                target_count: 10,
                enemy_speed: 300.0,
                spawn_interval: 0.3,
            },
            Wave {
                number: 2,
                defeated_count: 0,
                target_count: 10,
                enemy_speed: 500.0,
                spawn_interval: 0.1,
            },
        ],
        current_wave: 0,
    };

    commands.insert_resource(new_waves)
}

fn update_waves(
    mut waves: ResMut<Waves>,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let current_wave: Wave = waves.waves[waves.current_wave];

    if current_wave.defeated_count >= current_wave.target_count {
        if waves.current_wave == waves.waves.len() - 1 {
            enemy_timer.timer.pause();
            next_state.set(GameState::Clear);
            return;
        }

        waves.current_wave += 1;
        println!("Wave {}", waves.current_wave);
        let interval = waves.waves[waves.current_wave].spawn_interval;
        enemy_timer.timer = Timer::from_seconds(interval, TimerMode::Repeating);
    }
}
