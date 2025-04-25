use bevy::prelude::*;

use crate::GameState;
use crate::components::animation::AnimationConfig;
use crate::components::animation::AnimationType;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            execute_animation.run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_animations);
    }
}

pub fn execute_animation(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationConfig, &mut Sprite)>,
) {
    for (entity, mut config, mut sprite) in &mut query {
        config.timer.tick(time.delta());

        if !config.timer.just_finished() {
            continue;
        }

        if let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index >= config.last_sprite_index {
                if config.animation_type == AnimationType::Loop {
                    atlas.index = config.first_sprite_index;
                } else if config.animation_type == AnimationType::Once {
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            } else {
                atlas.index += 1;
            }
        }
    }
}

pub fn cleanup_animations(mut commands: Commands, mut query: Query<Entity, With<AnimationConfig>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
