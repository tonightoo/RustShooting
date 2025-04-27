use crate::GameState;
use crate::components::collider::*;
use crate::components::explosion::*;
use crate::components::item::*;
use crate::components::player::*;
use crate::components::score::Score;
use crate::components::wave::*;
use crate::systems::explosion::spawn_explosion;
use crate::systems::item::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

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
    mut player_query: Query<&mut Player>,
    mut item_query: Query<&mut ItemType>,
    mut commands: Commands,
    explosion: Res<ExplosionAsset>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
    explosion_sound: Res<ExplosionSound>,
    damage_sound: Res<DamageSound>,
    mut score: ResMut<Score>,
    mut waves: ResMut<Waves>,
    item_assets: Res<ItemAssets>,
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
                    if let Ok(mut player) = player_query.get_mut(e1) {
                        if player.invincible_timer.finished() {
                            player.hp -= 1;
                            player.invincible_timer = Timer::from_seconds(2.0, TimerMode::Once);

                            if player.hp <= 0 {
                                spawn_explosion(
                                    &mut commands,
                                    t1.translation.clone(),
                                    &explosion,
                                    ExplosionTag::Player,
                                );
                                audio.play(explosion_sound.sound.clone()).with_volume(0.2);
                                commands.entity(e1).despawn();
                            } else {
                                audio.play(damage_sound.sound.clone()).with_volume(0.2);
                            }
                        }
                    }
                }
                (ColliderTag::Enemy, ColliderTag::Player) => {
                    if let Ok(mut player) = player_query.get_mut(e2) {
                        if player.invincible_timer.finished() {
                            player.hp -= 1;
                            player.invincible_timer = Timer::from_seconds(2.0, TimerMode::Once);

                            if player.hp <= 0 {
                                spawn_explosion(
                                    &mut commands,
                                    t2.translation.clone(),
                                    &explosion,
                                    ExplosionTag::Player,
                                );
                                audio.play(explosion_sound.sound.clone()).with_volume(0.2);
                                commands.entity(e2).despawn();
                            } else {
                                audio.play(damage_sound.sound.clone()).with_volume(0.2);
                            }
                        }
                    }
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
                    audio.play(explosion_sound.sound.clone()).with_volume(0.2);
                    score.score += 100;
                    let current_wave = waves.current_wave;
                    waves.waves[current_wave].defeated_count += 1;

                    let mut rng = rand::rng();
                    let value = rng.random_range(0..100);
                    if value < 30 {
                        let item_type = match rng.random_range(0..3) {
                            0 => ItemType::RapidFire,
                            1 => ItemType::PiercingShot,
                            2 => ItemType::Shield,
                            _ => ItemType::RapidFire,
                        };
                        spawn_item(
                            &mut commands,
                            &item_assets,
                            item_type,
                            t1.translation.clone(),
                        );
                    }
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
                    audio.play(explosion_sound.sound.clone()).with_volume(0.2);
                    score.score += 100;
                    let current_wave = waves.current_wave;
                    waves.waves[current_wave].defeated_count += 1;

                    let mut rng = rand::rng();
                    let value = rng.random_range(0..100);
                    if value < 30 {
                        let item_type = match rng.random_range(0..3) {
                            0 => ItemType::RapidFire,
                            1 => ItemType::PiercingShot,
                            2 => ItemType::Shield,
                            _ => ItemType::RapidFire,
                        };
                        spawn_item(
                            &mut commands,
                            &item_assets,
                            item_type,
                            t1.translation.clone(),
                        );
                    }
                }
                (ColliderTag::Player, ColliderTag::Item) => {
                    if let Ok(item) = item_query.get(e2) {
                        if let Ok(mut player) = player_query.get_mut(e1) {
                            apply_item_effect(&mut player, *item);
                            commands.entity(e2).despawn_recursive();
                        }
                    }
                }
                (ColliderTag::Item, ColliderTag::Player) => {
                    if let Ok(item) = item_query.get(e1) {
                        if let Ok(mut player) = player_query.get_mut(e2) {
                            apply_item_effect(&mut player, *item);
                            commands.entity(e1).despawn_recursive();
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
