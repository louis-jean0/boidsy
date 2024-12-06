use bevy::prelude::*;
use bevy_spatial::kdtree::KDTree2;
use crate::boids_2d::components::Position;

#[derive(Component,Default)]
pub struct TrackedByKDTree;

pub type NNTree = KDTree2<TrackedByKDTree>;