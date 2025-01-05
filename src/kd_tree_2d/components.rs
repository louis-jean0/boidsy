use bevy::prelude::*;
use bevy_spatial::kdtree::KDTree2;

#[derive(Component,Default)]
pub struct TrackedByKDTree2D;

pub type NNTree2D = KDTree2<TrackedByKDTree2D>;