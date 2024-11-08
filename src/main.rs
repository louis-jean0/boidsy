use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,spawn_boids)
    .add_systems(Update, print_boids_types)
    .run();
}

pub fn spawn_boids(mut commands: Commands) {
    commands.spawn((Boid {
        boid_type: BoidType::Bird,
    },
    Position {
        position: Vec3::new(0.0,0.0,0.0),
    },
    Velocity {
        velocity: Vec3::new(0.0,0.0,0.0),
    },
    Acceleration {
        acceleration: Vec3::new(0.0,0.0,0.0) 
    }));
}

pub fn print_boids_types(boid_query: Query<&Boid>) {
    for boid in boid_query.iter() {
        let type_name = match boid.boid_type {
            BoidType::Bird => "Bird",
            BoidType::Fish => "Fish"
        };
        println!("Boid type : {}", type_name);
    }
}

#[derive (Component)]
pub struct Boid {
    pub boid_type: BoidType
}

#[derive (Component)]
pub struct Position {
    pub position: Vec3
}

#[derive (Component)]
pub struct Velocity {
    pub velocity: Vec3
}

#[derive (Component)]
pub struct Acceleration {
    pub acceleration: Vec3
}

#[derive (Debug)]
pub enum BoidType {
    Bird,
    Fish
}