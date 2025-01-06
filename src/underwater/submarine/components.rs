use bevy::prelude::*;

#[derive(Component)]
pub struct Submarine {
    pub speed: f32,
    pub turn_speed: f32,
    pub depth: f32,
}