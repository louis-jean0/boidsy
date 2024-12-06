use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_spatial::{AutomaticUpdate, SpatialStructure, TransformMode};
use components::TrackedByKDTree;

pub mod components;

pub struct KDTreePlugin;

impl Plugin for KDTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AutomaticUpdate::<TrackedByKDTree>::new()
        .with_spatial_ds(SpatialStructure::KDTree2)
        .with_frequency(Duration::from_millis(1))
        .with_transform(TransformMode::Transform));
    }
}