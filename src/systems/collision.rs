use crate::GameState;
use crate::components::assets::*;
use crate::components::collider::*;
use crate::components::explosion::*;
use crate::components::item::*;
use crate::components::player::*;
use crate::components::score::Score;
use crate::components::stage::*;
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
    item_query: Query<&mut ItemType>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    audio: Res<bevy_kira_audio::prelude::Audio>,
    mut score: ResMut<Score>,
    mut stage_db: ResMut<StageDatabase>,
    //mut waves: ResMut<Waves>,
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
                (ColliderTag::Player, ColliderTag::Enemy)
                | (ColliderTag::Player, ColliderTag::EnemyBullet) => {
                    handle_player_enemy_collision(
                        &mut commands,
                        &mut player_query,
                        &assets,
                        &audio,
                        e1.clone(),
                        t1.clone(),
                    );
                }
                (ColliderTag::Enemy, ColliderTag::Player)
                | (ColliderTag::EnemyBullet, ColliderTag::Player) => {
                    handle_player_enemy_collision(
                        &mut commands,
                        &mut player_query,
                        &assets,
                        &audio,
                        e2.clone(),
                        t2.clone(),
                    );
                }
                (ColliderTag::Enemy, ColliderTag::Bullet) => {
                    handle_enemy_bullet_collision(
                        &mut commands,
                        &mut player_query,
                        &assets,
                        &audio,
                        &mut score,
                        &mut stage_db,
                        //&mut waves,
                        e1.clone(),
                        e2.clone(),
                        t1.clone(),
                    );
                }
                (ColliderTag::Bullet, ColliderTag::Enemy) => {
                    handle_enemy_bullet_collision(
                        &mut commands,
                        &mut player_query,
                        &assets,
                        &audio,
                        &mut score,
                        &mut stage_db,
                        //&mut waves,
                        e2.clone(),
                        e1.clone(),
                        t2.clone(),
                    );
                }
                (ColliderTag::Player, ColliderTag::Item) => {
                    handle_player_item_collision(
                        &mut commands,
                        &item_query,
                        &mut player_query,
                        e1,
                        e2,
                    );
                }
                (ColliderTag::Item, ColliderTag::Player) => {
                    handle_player_item_collision(
                        &mut commands,
                        &item_query,
                        &mut player_query,
                        e2,
                        e1,
                    );
                }
                _ => {}
            }
        }
    }
}

fn handle_player_enemy_collision(
    commands: &mut Commands,
    player_query: &mut Query<&mut Player>,
    assets: &Res<GameAssets>,
    audio: &Res<bevy_kira_audio::prelude::Audio>,
    player_entity: Entity,
    player_transform: Transform,
) {
    if let Ok(mut player) = player_query.get_mut(player_entity) {
        if player.invincible_timer.finished() {
            player.hp -= 1;
            player.invincible_timer = Timer::from_seconds(2.0, TimerMode::Once);

            if player.hp <= 0 {
                spawn_explosion(
                    commands,
                    player_transform.translation.clone(),
                    &assets,
                    ExplosionTag::Player,
                );
                audio.play(assets.explosion_sound.clone()).with_volume(0.2);
                commands.entity(player_entity).despawn();
            } else {
                audio.play(assets.damage_sound.clone()).with_volume(0.2);
            }
        }
    }
}

fn handle_enemy_bullet_collision(
    commands: &mut Commands,
    player_query: &mut Query<&mut Player>,
    assets: &Res<GameAssets>,
    audio: &Res<bevy_kira_audio::prelude::Audio>,
    score: &mut ResMut<Score>,
    stage_db: &mut ResMut<StageDatabase>,
    //waves: &mut ResMut<Waves>,
    enemy_entity: Entity,
    bullet_entity: Entity,
    enemy_transform: Transform,
) {
    if let Ok(player) = player_query.single() {
        if !player.piercing {
            commands.entity(bullet_entity).despawn();
        }
    }
    commands.entity(enemy_entity).despawn();
    spawn_explosion(
        commands,
        enemy_transform.translation.clone(),
        &assets,
        ExplosionTag::Enemy,
    );
    audio.play(assets.explosion_sound.clone()).with_volume(0.2);
    score.score += 100;
    //let current_wave = waves.current_wave;
    //waves.waves[current_wave].defeated_count += 1;
    let stage_index = stage_db.current_index.clone();
    let wave_index = stage_db.settings[stage_index].current_index;
    stage_db.settings[stage_index].waves[wave_index].defeated_count += 1;

    let mut rng = rand::rng();
    let value = rng.random_range(0..100);
    if value < 30 {
        let item_type = match rng.random_range(0..3) {
            0 => ItemType::RapidFire,
            1 => ItemType::PiercingShot,
            2 => ItemType::Heal,
            _ => ItemType::RapidFire,
        };
        spawn_item(
            commands,
            &assets,
            item_type,
            enemy_transform.translation.clone(),
        );
    }
}

fn handle_player_item_collision(
    commands: &mut Commands,
    item_query: &Query<&mut ItemType>,
    player_query: &mut Query<&mut Player>,
    player_entity: Entity,
    item_entity: Entity,
) {
    if let Ok(item) = item_query.get(item_entity) {
        if let Ok(mut player) = player_query.get_mut(player_entity) {
            apply_item_effect(&mut player, *item);
            commands.entity(item_entity).despawn();
        }
    }
}
