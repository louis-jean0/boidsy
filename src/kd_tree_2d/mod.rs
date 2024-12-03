use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_spatial::{AutomaticUpdate, TransformMode, SpatialAccess};
pub mod components;
pub use components::*;

pub struct KDTreePlugin;

impl Plugin for KDTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AutomaticUpdate::<TrackedByKDTree>::new()
        .with_frequency(Duration::from_secs_f32(0.3))
        .with_transform(TransformMode::Component::<boids_2d::components::Position>()));
    }
}