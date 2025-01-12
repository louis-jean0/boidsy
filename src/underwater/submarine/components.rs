use bevy::prelude::*;

#[derive(Component)]
pub struct Submarine {
    pub speed: f32,
    pub turn_speed: f32,
    pub vertical_speed: f32,
    pub max_pitch: f32,
    pub current_pitch: f32,
}

impl Default for Submarine {
    fn default() -> Self {
        Self {
            speed: 20.0,
            turn_speed: 2.0,
            vertical_speed: 10.0,
            max_pitch: 0.5,  // About 30 degrees in radians
            current_pitch: 0.0,
        }
    }
}

#[derive(Component)]
pub struct SubmarineCamera {
    pub follow_distance: f32,
    pub height_offset: f32,
}

impl Default for SubmarineCamera {
    fn default() -> Self {
        Self {
            follow_distance: 30.0,
            height_offset: 10.0,
        }
    }
}