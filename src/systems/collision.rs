use crate::GameState;
use crate::components::collider::*;
use crate::components::explosion::*;
use crate::components::score::Score;
use crate::components::wave::*;
use crate::systems::explosion::spawn_explosion;
use crate::systems::wave::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_system.run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn collision_system(
    query: Query<(Entity, &Transform, &Collider)>,
    mut commands: Commands,
    explosion: Res<ExplosionAsset>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
    sound_asset: Res<ExplosionSound>,
    mut score: ResMut<Score>,
    mut waves: ResMut<Waves>,
) {
    let mut pairs = query.iter_combinations::<2>();

    while let Some([(e1, t1, c1), (e2, t2, c2)]) = pairs.fetch_next() {
        let collision = match (&c1.shape, &c2.shape) {
            (ColliderShape::Circle { radius: r1 }, ColliderShape::Circle { radius: r2 }) => {
                let dist = t1.translation.distance(t2.translation);
                dist < (r1 + r2)
            }
            (ColliderShape::Rectangle { size: s1 }, ColliderShape::Rectangle { size: s2 })
            | (ColliderShape::Capsule { size: s1 }, ColliderShape::Capsule { size: s2 }) => {
                let pos1 = t1.translation.xy();
                let pos2 = t2.translation.xy();
                let half1 = *s1 / 2.0;
                let half2 = *s2 / 2.0;

                (pos1.x - pos2.x).abs() < (half1.x + half2.x)
                    && (pos1.y - pos2.y).abs() < (half1.y + half2.y)
            }
            _ => false,
        };

        if collision {
            match (c1.tag, c2.tag) {
                (ColliderTag::Player, ColliderTag::Enemy) => {
                    spawn_explosion(
                        &mut commands,
                        t1.translation.clone(),
                        &explosion,
                        ExplosionTag::Player,
                    );
                    audio.play(sound_asset.sound.clone()).with_volume(0.2);
                    commands.entity(e1).despawn();
                }
                (ColliderTag::Enemy, ColliderTag::Player) => {
                    spawn_explosion(
                        &mut commands,
                        t2.translation.clone(),
                        &explosion,
                        ExplosionTag::Player,
                    );
                    audio.play(sound_asset.sound.clone()).with_volume(0.2);
                    commands.entity(e2).despawn();
                }
                (ColliderTag::Enemy, ColliderTag::Bullet) => {
                    commands.entity(e1).despawn();
                    commands.entity(e2).despawn();
                    spawn_explosion(
                        &mut commands,
                        t1.translation.clone(),
                        &explosion,
                        ExplosionTag::Enemy,
                    );
                    audio.play(sound_asset.sound.clone()).with_volume(0.2);
                    score.score += 100;
                    let current_wave = waves.current_wave;
                    waves.waves[current_wave].defeated_count += 1;
                }
                (ColliderTag::Bullet, ColliderTag::Enemy) => {
                    commands.entity(e1).despawn();
                    commands.entity(e2).despawn();
                    spawn_explosion(
                        &mut commands,
                        t2.translation.clone(),
                        &explosion,
                        ExplosionTag::Enemy,
                    );
                    audio.play(sound_asset.sound.clone()).with_volume(0.2);
                    score.score += 100;
                    let current_wave = waves.current_wave;
                    waves.waves[current_wave].defeated_count += 1;
                }
                _ => {}
            }
        }
    }
}
