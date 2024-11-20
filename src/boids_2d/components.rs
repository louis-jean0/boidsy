use bevy::prelude::*;

#[derive(Debug)]
pub enum BoidType {
    Bird,
    Fish
}

#[derive(Component, Debug)]
pub struct Boid {
    pub boid_type: BoidType
}

#[derive(Component, Debug)]
pub struct Position {
    pub position: Vec2
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub velocity: Vec2
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub acceleration: Vec2
}