use crate::GameState;
use crate::components::collider::*;
use crate::components::explosion::ExplosionAsset;
use crate::systems::explosion::spawn_explosion;
use bevy::prelude::*;

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
                | (ColliderTag::Enemy, ColliderTag::Player) => {
                    //damage
                }
                (ColliderTag::Enemy, ColliderTag::Bullet) => {
                    commands.entity(e1).despawn();
                    commands.entity(e2).despawn();
                    spawn_explosion(&mut commands, t1.translation.clone(), &explosion);
                }
                (ColliderTag::Bullet, ColliderTag::Enemy) => {
                    commands.entity(e1).despawn();
                    commands.entity(e2).despawn();
                    spawn_explosion(&mut commands, t2.translation.clone(), &explosion);
                }
                _ => {}
            }
        }
    }
}
