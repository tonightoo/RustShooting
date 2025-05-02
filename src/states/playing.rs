use crate::GameState;
use crate::components::assets::*;
use crate::components::explosion::ExplosionTag;
use crate::components::player::*;
use crate::components::stage::*;
use crate::components::wave::*;
use crate::systems::animation::AnimationPlugin;
use crate::systems::assets::*;
use crate::systems::bullet::BulletPlugin;
use crate::systems::collision::CollisionPlugin;
use crate::systems::enemy::EnemyPlugin;
use crate::systems::item::ItemPlugin;
use crate::systems::player::PlayerPlugin;
use crate::systems::score::ScorePlugin;
use crate::systems::sets::MySystemSet;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), initialize_background)
            .add_systems(
                OnEnter(GameState::Playing),
                start_bgm.after(MySystemSet::LoadAssets),
            )
            .add_plugins(AssetsPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(BulletPlugin)
            .add_plugins(AnimationPlugin)
            .add_plugins(CollisionPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(ItemPlugin)
            .add_systems(Update, playing_system.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), despawn_gameover_text)
            .add_systems(OnExit(GameState::Playing), stop_bgm)
            .add_systems(OnExit(GameState::Playing), despawn_background);
    }
}

fn initialize_background(mut commands: Commands, stage_db: Res<StageDatabase>) {
    //commands.insert_resource(ClearColor(Color::srgb(0.7, 0.44, 0.25)));
    commands.insert_resource(DeadTimer {
        timer: Timer::from_seconds(0.0, TimerMode::Once),
    });

    commands.spawn((
        Sprite::from_image(
            stage_db.settings[stage_db.current_index]
                .background_image
                .clone(),
        ),
        Transform::from_xyz(0.0, 0.0, -1.0),
        BackgroundImage,
    ));
}

fn start_bgm(assets: Res<GameAssets>, audio: Res<bevy_kira_audio::prelude::Audio>) {
    audio
        .play(assets.playing_bgm.clone())
        .with_volume(0.2)
        .looped();
}

fn playing_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    query_player: Query<Entity, With<Player>>,
    query_explosion: Query<&ExplosionTag>,
    mut timer: ResMut<DeadTimer>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let player_explosion_exists = query_explosion
        .iter()
        .any(|e| matches!(e, ExplosionTag::Player));

    if query_player.is_empty() && !player_explosion_exists {
        if timer.timer.elapsed() == Duration::ZERO {
            timer.timer = Timer::from_seconds(3.0, TimerMode::Once);
            println!("timer set");
        }

        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            commands.spawn((
                Text::new("Game Over"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    position_type: PositionType::Absolute,
                    //justify_content: JustifyContent::Center,
                    //align_items: AlignItems::Center,
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    //box_sizing: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    display: Display::Flex,
                    //top: Val::Percent(60.0),
                    //left: Val::Percent(20.0),
                    ..default()
                },
                GameOverText,
            ));

            timer.timer.pause();
            if keyboard.just_pressed(KeyCode::Space) {
                next_state.set(GameState::Title);
            }
        }
    }
    //println!("Playing: Game is on!");
    //if keyboard.just_pressed(KeyCode::Escape) {
    //    next_state.set(GameState::GameOver);
    //}
}

fn despawn_gameover_text(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn despawn_background(mut commands: Commands, query: Query<Entity, With<BackgroundImage>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn stop_bgm(audio: Res<bevy_kira_audio::prelude::Audio>) {
    audio.stop();
}
