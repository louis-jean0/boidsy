use bevy::ecs::entity;
use bevy::prelude::*;
use bevy::render::texture;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

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

#[derive(Bundle)]
pub struct BoidBundle {
    pub boid: Boid,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub sprite_bundle: SpriteBundle
}

#[derive(Resource)]
pub struct BoidSettings {
    pub count: usize,
    pub cohesion_range: f32,
    pub repulsion_range: f32,
    pub alignment_range: f32,
    pub boid_type: BoidType
}

impl BoidSettings {
    pub fn default() -> Self {
        BoidSettings {
            count: 50,
            cohesion_range: 50.0,
            repulsion_range: 10.0,
            alignment_range: 50.0,
            boid_type: BoidType::Fish
        }
    }

    pub fn new(count: usize, cohesion_range: f32, repulsion_range: f32, alignment_range: f32, boid_type: BoidType) -> Self {
        BoidSettings {
            count: count,
            cohesion_range: cohesion_range,
            repulsion_range: repulsion_range,
            alignment_range: alignment_range,
            boid_type: boid_type
        }
    }
}

pub fn spawn_boid(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    boid_settings: Res<BoidSettings>) {

    let window = window_query.get_single().unwrap();
    let texture_path = match boid_settings.boid_type {
        BoidType::Bird => "../assets/bird.png",
        BoidType::Fish => "../assets/fish.png"
    };

    for _ in 0..boid_settings.count {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn(
            BoidBundle {
                boid: Boid {
                    boid_type: BoidType::Fish
                },
                position: Position {
                    position: Vec2::new(random_x, random_y)
                },
                velocity: Velocity {
                    velocity: Vec2::new(0.0, 0.0)
                },
                acceleration: Acceleration {
                    acceleration: Vec2::new(0.0, 0.0)
                },
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(random_x, random_y, 0.0),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::new(0.1,0.1,0.1)
                    },
                    texture: asset_server.load(texture_path),
                    ..default()
                }
            }
        );
    }
}

pub fn flocking(
    mut boid_query_1: Query<(Entity, &Position, &Velocity, &mut Acceleration), With<Boid>>,
    boid_query_2: Query<(Entity, &Position, &Velocity), With<Boid>>,
    boid_settings: Res<BoidSettings>
) {
    let cohesion_range = boid_settings.cohesion_range;
    let repulsion_range = boid_settings.repulsion_range;
    let alignment_range = boid_settings.alignment_range;

    for(entity, position, velocity, mut acceleration) in boid_query_1.iter_mut() {
        let mut cohesion_neighbors: Vec<(Entity, &Position)> = Vec::new();
        let mut repulsion_neighbors: Vec<(Entity, &Position)> = Vec::new();
        let mut alignment_neighbors: Vec<(Entity, &Velocity)> = Vec::new();

        for (other_entity, other_position, other_velocity) in boid_query_2.iter() {
            if entity == other_entity {
                continue;
            }
            let distance = position.position.distance(other_position.position);
            if distance < cohesion_range && distance > repulsion_range {
                cohesion_neighbors.push((other_entity, other_position));
            }
            if distance < repulsion_range {
                repulsion_neighbors.push((other_entity, other_position));
            }
            if distance < alignment_range {
                alignment_neighbors.push((other_entity, other_velocity));
            }
        }
        let cohesion_force: Vec2 = cohesion(position, &cohesion_neighbors);
        let avoidance_force: Vec2 = avoidance(position, &repulsion_neighbors);
        let alignment_force: Vec2 = alignment(velocity, &alignment_neighbors);

        acceleration.acceleration += cohesion_force + avoidance_force + alignment_force;
    }
}


pub fn cohesion(position: &Position, cohesion_neighbors: &Vec<(Entity, &Position)>) -> Vec2 {
    Vec2::default()
}

pub fn avoidance(position: &Position, repulsion_neighbors: &Vec<(Entity, &Position)>) -> Vec2 {
    Vec2::default()
}

pub fn alignment(velocity: &Velocity, alignment_neighbors: &Vec<(Entity, &Velocity)>) -> Vec2 {
    Vec2::default()
}

pub fn update_boid_position(
    mut boid_query: Query<(&mut Position, &mut Velocity, &mut Acceleration), With<Boid>>,
    delta_time: f32
) {
    for(mut position, mut velocity, mut acceleration) in boid_query.iter_mut() {
        velocity.velocity += acceleration.acceleration * delta_time;
        position.position += velocity.velocity * delta_time;
        acceleration.acceleration = Vec2::ZERO;
    }
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