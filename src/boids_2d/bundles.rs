use bevy::prelude::*;
use crate::boids_2d::components::*;

#[derive(Bundle)]
pub struct BoidBundle {
    pub boid: Boid,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub sprite_bundle: SpriteBundle
}

#[derive(Bundle)]
pub struct ObstacleBundle {
    pub position: Position,
}

