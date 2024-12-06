use bevy::prelude::*;
use bevy_spatial::kdtree::KDTree2;

#[derive(Component,Default)]
pub struct TrackedByKDTree;

pub type NNTree = KDTree2<TrackedByKDTree>;