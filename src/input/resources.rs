use bevy::prelude::*;

#[derive(Resource)]
pub struct ShapeSettings {
    pub radius: f32,
}

impl Default for ShapeSettings {
    fn default() -> Self {
        ShapeSettings {
            radius: 10.,
        }
    }
}

impl ShapeSettings {
    pub fn new(radius: f32) -> Self {
        ShapeSettings {
            radius: radius,
            ..Default::default()
        }
    }
}