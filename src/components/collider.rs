use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub shape: ColliderShape,
    pub tag: ColliderTag,
}

#[derive(Copy, Clone)]
pub enum ColliderShape {
    Circle { radius: f32 },
    Rectangle { size: Vec2 },
    Capsule { size: Vec2 }, // width: size.x, height: size.y
}

#[derive(Copy, Clone)]
pub enum ColliderTag {
    Player,
    Enemy,
    Bullet,
    Item,
}
