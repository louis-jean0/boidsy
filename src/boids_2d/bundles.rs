use bevy::prelude::*;
use crate::boids_2d::components::*;
use crate::kd_tree_2d::components::*;
#[derive(Bundle)]
pub struct BoidBundle {
    pub boid: Boid,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub sprite_bundle: SpriteBundle,
    pub tracked_by_kdtree: TrackedByKDTree
}

#[derive(Bundle)]
pub struct ObstacleBundle {
    pub position: Position,
}

