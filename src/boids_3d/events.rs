use bevy::prelude::*;

#[derive(Event)]
pub struct ApplyForceEvent {
    pub entity: Entity,
    pub force: Vec3
}

#[derive(Event)]
pub struct ResizeEvent {
    pub scale: f32
}