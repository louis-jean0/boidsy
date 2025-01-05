use bevy::prelude::*;
use crate::boids_2d::components::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::kd_tree_2d::components::*;

#[derive(Bundle)]
pub struct BoidBundle {
    pub boid: Boid,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub sprite_bundle: SpriteBundle,
    pub mode_2d_marker: Mode2DMarker,
    pub tracked_by_kdtree: TrackedByKDTree2D
}

#[derive(Bundle)]
pub struct ObstacleBundle {
    pub material_mesh: MaterialMesh2dBundle<ColorMaterial>,
}