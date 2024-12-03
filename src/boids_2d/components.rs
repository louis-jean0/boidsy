use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component, Debug)]
pub struct Boid {
    pub group: u8
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
