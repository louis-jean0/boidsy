use bevy::prelude::*;

#[derive(Resource)]
pub struct ShapeSettings {
    pub radius: f32,
}

impl Default for ShapeSettings {
    fn default() -> Self {
        ShapeSettings {
            radius: 50.0,
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

#[derive(Resource)]
pub struct MouseSettings {
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.001,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}