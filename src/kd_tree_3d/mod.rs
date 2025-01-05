use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_spatial::{AutomaticUpdate, SpatialStructure, TransformMode};
use components::TrackedByKDTree3D;

pub mod components;

pub struct KDTree3DPlugin;

impl Plugin for KDTree3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AutomaticUpdate::<TrackedByKDTree3D>::new()
        .with_spatial_ds(SpatialStructure::KDTree3)
        .with_frequency(Duration::from_millis(1))
        .with_transform(TransformMode::Transform));
    }
}