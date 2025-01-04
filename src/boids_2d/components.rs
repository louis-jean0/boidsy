use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Boid {
    pub group: u8
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub velocity: Vec2
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub acceleration: Vec2
}

#[derive(Component)]
pub struct Shark;
