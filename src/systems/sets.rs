use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MySystemSet {
    LoadAssets,
    SpawnEntities,
    Cleanup,
}
