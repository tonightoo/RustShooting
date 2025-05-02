use crate::GameState;
use crate::components::assets::*;
use crate::components::enemy::EnemySpawnTimer;
use crate::components::stage::*;
use crate::components::wave::*;
use crate::systems::sets::MySystemSet;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct StageSelectPlugin;

impl Plugin for StageSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::StageSelect),
            load_stages.in_set(MySystemSet::LoadAssets),
        )
        .add_systems(
            OnEnter(GameState::StageSelect),
            setup_stage_select.after(MySystemSet::LoadAssets),
        )
        .add_systems(
            Update,
            update_stage_select.run_if(in_state(GameState::StageSelect)),
        )
        .add_systems(
            Update,
            stage_select_input.run_if(in_state(GameState::StageSelect)),
        )
        .add_systems(Update, update_waves.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::StageSelect), cleanup_stage_select);
    }
}

fn load_stages(mut commands: Commands, assets: Res<GameAssets>) {
    let stage_db: StageDatabase = StageDatabase {
        settings: vec![
            StageSetting {
                name: String::from("Stage1"),
                background_image: assets.ground_texture.clone(),
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
                        target_count: 20,
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
                current_index: 0,
            },
            StageSetting {
                name: String::from("Stage2"),
                background_image: assets.ocean_texture.clone(),
                waves: vec![
                    Wave {
                        number: 0,
                        defeated_count: 0,
                        target_count: 10,
                        enemy_speed: 100.0,
                        spawn_interval: 0.1,
                    },
                    Wave {
                        number: 1,
                        defeated_count: 0,
                        target_count: 20,
                        enemy_speed: 200.0,
                        spawn_interval: 0.1,
                    },
                    Wave {
                        number: 2,
                        defeated_count: 0,
                        target_count: 10,
                        enemy_speed: 500.0,
                        spawn_interval: 0.1,
                    },
                ],
                current_index: 0,
            },
            StageSetting {
                name: String::from("Stage3"),
                background_image: assets.universe_texture.clone(),
                waves: vec![
                    Wave {
                        number: 0,
                        defeated_count: 0,
                        target_count: 10,
                        enemy_speed: 500.0,
                        spawn_interval: 2.0,
                    },
                    Wave {
                        number: 1,
                        defeated_count: 0,
                        target_count: 20,
                        enemy_speed: 500.0,
                        spawn_interval: 1.0,
                    },
                    Wave {
                        number: 2,
                        defeated_count: 0,
                        target_count: 10,
                        enemy_speed: 500.0,
                        spawn_interval: 0.1,
                    },
                ],
                current_index: 0,
            },
        ],
        current_index: 0,
    };
    commands.insert_resource(stage_db);
}

fn setup_stage_select(mut commands: Commands, stage_db: Res<StageDatabase>) {
    let disp_text: String = create_disp_text(&stage_db);

    commands.spawn((
        Text::new(disp_text),
        StageSelectScreenTag,
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            display: Display::Flex,
            ..default()
        },
    ));
}

fn update_stage_select(
    stage_db: Res<StageDatabase>,
    query: Query<Entity, With<StageSelectScreenTag>>,
    mut writer: TextUiWriter,
) {
    let new_text: String = create_disp_text(&stage_db);
    for entity in query.iter() {
        *writer.text(entity, 0) = new_text.clone();
    }
}

fn create_disp_text(stage_db: &Res<StageDatabase>) -> String {
    let mut disp_text: String = String::new();

    for i in 0..stage_db.settings.len() {
        if i == stage_db.current_index {
            disp_text = format!("{} > {}\n\n", disp_text, stage_db.settings[i].name);
        } else {
            disp_text = format!("{}   {}\n\n", disp_text, stage_db.settings[i].name);
        }
    }

    return disp_text;
}

fn stage_select_input(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut stage_db: ResMut<StageDatabase>,
) {
    let stage_num: isize = stage_db.settings.len() as isize;
    let index: isize = stage_db.current_index as isize;
    if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyK) {
        stage_db.current_index = ((index - 1 + stage_num) % stage_num) as usize;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyJ)
    {
        stage_db.current_index = ((index + 1 + stage_num) % stage_num) as usize;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn update_waves(
    mut stage_db: ResMut<StageDatabase>,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let stage_index = stage_db.current_index;
    let mut wave_index = stage_db.settings[stage_index].current_index;
    let wave_length = stage_db.settings[stage_index].waves.len();
    let wave = stage_db.settings[stage_index].waves[wave_index];

    if wave.defeated_count >= wave.target_count {
        if wave_index == wave_length - 1 {
            enemy_timer.timer.pause();
            next_state.set(GameState::Clear);
            return;
        }

        stage_db.settings[stage_index].current_index += 1;
        wave_index += 1;
        let interval = stage_db.settings[stage_index].waves[wave_index].spawn_interval;
        enemy_timer.timer = Timer::from_seconds(interval, TimerMode::Repeating);
    }
}

fn cleanup_stage_select(mut commands: Commands, query: Query<Entity, With<StageSelectScreenTag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
