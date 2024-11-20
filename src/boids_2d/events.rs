use bevy::prelude::*;

#[derive(Event)]
pub struct ApplyForceEvent {
    pub entity: Entity,
    pub force: Vec2
}