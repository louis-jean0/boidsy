use bevy::prelude::*;
use crate::boids_2d::components::*;
<<<<<<< HEAD
use crate::kd_tree_2d::components::*;
=======
use bevy::sprite::MaterialMesh2dBundle;
>>>>>>> Ben

#[derive(Bundle)]
pub struct BoidBundle {
    pub boid: Boid,
    //pub transform: Transform,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub sprite_bundle: SpriteBundle,
    pub tracked_by_kdtree: TrackedByKDTree
}

#[derive(Bundle)]
pub struct ObstacleBundle {
<<<<<<< HEAD
    pub transform: Transform,
=======
    pub position: Position,
    pub material_mesh: MaterialMesh2dBundle<ColorMaterial>,
>>>>>>> Ben
}


