use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_spatial::{AutomaticUpdate, SpatialStructure, TransformMode};
use components::TrackedByKDTree2D;

pub mod components;

pub struct KDTree2DPlugin;

impl Plugin for KDTree2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AutomaticUpdate::<TrackedByKDTree2D>::new()
        .with_spatial_ds(SpatialStructure::KDTree2)
        .with_frequency(Duration::from_millis(1))
        .with_transform(TransformMode::Transform));
    }
}