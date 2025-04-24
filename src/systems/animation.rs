use bevy::prelude::*;

use crate::components::animation::AnimationConfig;
use crate::components::animation::AnimationType;

fn execute_animation(
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
            if atlas.index == config.last_sprite_index {
                if config.animation_type == AnimationType::Once {
                    atlas.index = config.first_sprite_index;
                } else if config.animation_type == AnimationType::Loop {
                    commands.entity(entity).despawn();
                }
            } else {
                atlas.index += 1;
            }
        }
    }
}
