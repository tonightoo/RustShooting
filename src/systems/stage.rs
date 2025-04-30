use crate::GameState;
use crate::components::stage::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct StageSelectPlugin;

impl Plugin for StageSelectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StageSelection {
            stages: vec![Stage::Stage1, Stage::Stage2, Stage::Stage3],
            current_index: 0,
        })
        .add_systems(OnEnter(GameState::StageSelect), setup_stage_select)
        .add_systems(
            Update,
            update_stage_select.run_if(in_state(GameState::StageSelect)),
        )
        .add_systems(
            Update,
            stage_select_input.run_if(in_state(GameState::StageSelect)),
        )
        .add_systems(OnExit(GameState::StageSelect), cleanup_stage_select);
    }
}

fn setup_stage_select(mut commands: Commands, stage_selection: Res<StageSelection>) {
    let disp_text: String = create_disp_text(&stage_selection);

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
    selection: Res<StageSelection>,
    query: Query<Entity, With<StageSelectScreenTag>>,
    mut writer: TextUiWriter,
) {
    let new_text: String = create_disp_text(&selection);
    for entity in query.iter() {
        *writer.text(entity, 0) = new_text.clone();
    }
}

fn create_disp_text(stage_selection: &Res<StageSelection>) -> String {
    let mut disp_text: String = String::new();
    for i in 0..stage_selection.stages.len() {
        let stage_name: String = get_stage_name(stage_selection.stages[i]);
        if i == stage_selection.current_index {
            disp_text = format!("{} > {}\n\n", disp_text, stage_name);
        } else {
            disp_text = format!("{}   {}\n\n", disp_text, stage_name);
        }
    }

    return disp_text;
}

fn get_stage_name(stage: Stage) -> String {
    match stage {
        Stage::Stage1 => String::from("Stage1"),
        Stage::Stage2 => String::from("Stage2"),
        Stage::Stage3 => String::from("Stage3"),
    }
}

fn stage_select_input(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selection: ResMut<StageSelection>,
) {
    let stage_num: isize = selection.stages.len() as isize;
    let index: isize = selection.current_index as isize;
    if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyK) {
        selection.current_index = ((index - 1 + stage_num) % stage_num) as usize;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyJ)
    {
        selection.current_index = ((index + 1 + stage_num) % stage_num) as usize;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn cleanup_stage_select(mut commands: Commands, query: Query<Entity, With<StageSelectScreenTag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
