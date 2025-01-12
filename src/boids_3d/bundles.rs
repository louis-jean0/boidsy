use bevy::prelude::*;
use crate::boids_3d::components::*;
use crate::kd_tree_3d::components::*;

#[derive(Bundle)]
pub struct BoidBundle {
    pub boid: Boid,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub pbr_bundle: PbrBundle,
    pub tracked_by_kdtree: TrackedByKDTree3D
}

#[derive(Bundle)]
pub struct ObstacleBundle {
    pub pbr_bundle: PbrBundle
}