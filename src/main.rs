use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .run();
}

// pub fn setup(mut commands: Commands) {
//     commands.spawn()
// }

#[derive (Component)]
pub struct Boid;

#[derive (Component)]
pub struct Velocity {
    pub velocity: Vec3
}

#[derive (Component)]
pub struct Position {
    pub position: Vec3
}

#[derive (Component)]
pub struct Acceleration {
    pub acceleration: Vec3
}

#[derive (Debug)]
pub enum BoidType {
    Bird,
    Fishc
}