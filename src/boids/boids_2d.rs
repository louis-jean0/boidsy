use bevy::ecs::{entity, event};
use bevy::prelude::*;
use bevy::render::texture;
use bevy::ui::debug;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use system_adapter::new;

pub const SPRITE_SIZE: f32 = 32.0;
pub const TARGET_BOID_ID_DEBUG: u32 = 42;

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

#[derive(Event)]
pub struct ApplyForceEvent {
    pub entity: Entity,
    pub force: Vec2
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
    pub visual_range: f32,
    pub separation_range: f32,
    pub protected_range: f32,
    pub cohesion_coeff: f32,
    pub alignment_coeff: f32,
    pub separation_coeff: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub boid_type: BoidType
}

impl Default for BoidSettings {
    fn default() -> Self {
        BoidSettings {
            count: 500,
            visual_range: 20.0,
            separation_range: 10.0,
            protected_range: 5.0,
            cohesion_coeff: 20.0,
            alignment_coeff: 5.0,
            separation_coeff: 3.0,
            min_speed: 200.0,
            max_speed: 500.0,
            boid_type: BoidType::Fish
        }
    }
}

impl BoidSettings {
    pub fn new(count: usize, visual_range: f32, separation_range: f32, boid_type: BoidType) -> Self {
        BoidSettings {
            count: count,
            visual_range: visual_range,
            separation_range: separation_range,
            boid_type: boid_type,
            ..Default::default()
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
        BoidType::Bird => "../assets/resize_test.png",
        BoidType::Fish => "../assets/fish.png"
    };

    for _ in 0..boid_settings.count {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        let random_angle = random::<f32>() * 360.0 * (std::f32::consts::PI / 180.0); // En radians
        commands.spawn(
            BoidBundle {
                boid: Boid {
                    boid_type: BoidType::Fish
                },
                position: Position {
                    position: Vec2::new(random_x, random_y)
                },
                velocity: Velocity {
                    velocity: Vec2::new(f32::cos(random_angle), f32::sin(random_angle))
                },
                acceleration: Acceleration {
                    acceleration: Vec2::new(0.0,0.0)
                },
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(random_x, random_y, 0.0),
                        rotation: Quat::IDENTITY,
                        ..default()
                    },
                    texture: asset_server.load(texture_path),
                    ..default()
                }
            }
        );
    }
}

pub fn flocking(
    boid_query: Query<(Entity, &Position, &Velocity), With<Boid>>,
    mut event_writer: EventWriter<ApplyForceEvent>,
    boid_settings: Res<BoidSettings>
) {
    let visual_range = boid_settings.visual_range;
    let separation_range = boid_settings.separation_range;

    for(entity, position, velocity) in boid_query.iter() {
        let mut cohesion_neighbors: Vec<(Entity, &Position)> = Vec::new();
        let mut repulsion_neighbors: Vec<(Entity, &Position)> = Vec::new();
        let mut alignment_neighbors: Vec<(Entity, &Velocity)> = Vec::new();
        for (other_entity, other_position, other_velocity) in boid_query.iter() {
            if entity == other_entity {
                continue;
            }
            let distance = position.position.abs().distance(other_position.position.abs());
            if distance < separation_range {
                repulsion_neighbors.push((other_entity, other_position));
            }
            else if distance < visual_range && distance > separation_range {
                cohesion_neighbors.push((other_entity, other_position));
                alignment_neighbors.push((other_entity, other_velocity));
            }
        }
        let cohesion_force: Vec2 = cohesion(position, &cohesion_neighbors, boid_settings.cohesion_coeff);
        let avoidance_force: Vec2 = avoidance(position, &repulsion_neighbors, boid_settings.separation_coeff);
        let alignment_force: Vec2 = alignment(&velocity, &alignment_neighbors, boid_settings.alignment_coeff);
        let total_force = cohesion_force + avoidance_force + alignment_force;
        event_writer.send(ApplyForceEvent {
            entity: entity,
            force: total_force
        });

        // // DEBUG
        // if is_target_boid_debug {
        //     *texture_handle = debug_texture.clone();
        //     println!("\n=== Débogage du boid {} ===", TARGET_BOID_ID_DEBUG);
        //     println!("Position: {:?}", position.position);
        //     println!("Vitesse: {:?}", velocity.velocity);
        //     println!("Cohesion force: {:?}", cohesion_force);
        //     println!("Avoidance force: {:?}", avoidance_force);
        //     println!("Alignment force: {:?}", alignment_force);

        //     println!("Voisins pour cohesion:");
        //     for (neighbor_entity, neighbor_position) in &cohesion_neighbors {
        //         println!("- Boid {} à la position {:?}", neighbor_entity.index(), neighbor_position.position);
        //     }

        //     println!("Voisins pour avoidance:");
        //     for (neighbor_entity, neighbor_position) in &repulsion_neighbors {
        //         println!("- Boid {} à la position {:?}", neighbor_entity.index(), neighbor_position.position);
        //     }

        //     println!("Voisins pour alignment:");
        //     for (neighbor_entity, neighbor_velocity) in &alignment_neighbors {
        //         println!("- Boid {} avec la vitesse {:?}", neighbor_entity.index(), neighbor_velocity.velocity);
        //     }
        // }
    }
}

pub fn cohesion(position: &Position, cohesion_neighbors: &Vec<(Entity, &Position)>, cohesion_coeff: f32) -> Vec2 {
    let mut cohesion_force: Vec2 = Vec2::ZERO;
    let nb_neighbors = cohesion_neighbors.len();
    if nb_neighbors == 0 {
        return Vec2::ZERO;
    }
    for (_, other_position) in cohesion_neighbors.iter() {
        cohesion_force.x += other_position.position.x;
        cohesion_force.y += other_position.position.y;
    }
    cohesion_force /= nb_neighbors as f32;
    cohesion_force = cohesion_force - position.position;
    // if cohesion_force.length() > 0.0 {
    //     cohesion_force = cohesion_force.normalize();
    // }
    cohesion_force * cohesion_coeff
}

pub fn avoidance(position: &Position, repulsion_neighbors: &Vec<(Entity, &Position)>, separation_coeff: f32) -> Vec2 {
    let mut avoidance_force: Vec2 = Vec2::ZERO;
    for (_, other_position) in repulsion_neighbors.iter() {
        avoidance_force += position.position - other_position.position;
    }
    // if avoidance_force.length() > 0.0 {
    //     avoidance_force = avoidance_force.normalize();
    // }
    avoidance_force * separation_coeff
}

pub fn alignment(velocity: &Velocity, alignment_neighbors: &Vec<(Entity, &Velocity)>, alignment_coeff: f32) -> Vec2 {
    let mut alignment_force: Vec2 = Vec2::ZERO;
    let nb_neighbors = alignment_neighbors.len();
    if nb_neighbors == 0 {
        return Vec2::ZERO;
    }
    for (_, other_velocity) in alignment_neighbors.iter() {
        alignment_force.x += other_velocity.velocity.x;
        alignment_force.y += other_velocity.velocity.y;
    }
    alignment_force /= nb_neighbors as f32;
    alignment_force = alignment_force - velocity.velocity;
    // if alignment_force.length() > 0.0 {
    //     alignment_force = alignment_force.normalize();
    // }
    alignment_force * alignment_coeff
}

pub fn apply_forces_system(
    mut forces: EventReader<ApplyForceEvent>,
    mut boid_query: Query<&mut Acceleration, With<Boid>>
) {
    for ApplyForceEvent{entity, force} in forces.read() {
        if let Ok(mut acceleration) = boid_query.get_mut(*entity) {
            acceleration.acceleration += *force;
            println!("{:?}", acceleration.acceleration);
        }
    } 
}

pub fn update_boid_position(
    mut boid_query: Query<(&mut Position, &mut Velocity, &mut Acceleration, &mut Transform), With<Boid>>,
    boid_settings: Res<BoidSettings>,
    time: Res<Time>
) {
    for(mut position, mut velocity, mut acceleration, mut transform) in boid_query.iter_mut() {
        velocity.velocity += acceleration.acceleration * time.delta_seconds();
        if velocity.velocity.length() < boid_settings.min_speed {
            velocity.velocity = velocity.velocity.normalize() * boid_settings.min_speed;
        }
        if velocity.velocity.length() > boid_settings.max_speed {
            velocity.velocity = velocity.velocity.normalize() * boid_settings.max_speed;
        }
        position.position += velocity.velocity * time.delta_seconds();
        transform.translation = Vec3::new(position.position.x, position.position.y, 0.0); // Pour faire bouger le sprite en lui-même
        acceleration.acceleration = Vec2::ZERO;
        transform.rotation = Quat::from_rotation_z(-velocity.velocity.x.atan2(velocity.velocity.y));
    }
}

pub fn confine_movement (
    mut boid_query: Query<(&mut Position, &mut Velocity, &mut Acceleration), With<Boid>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    let half_sprite_size = SPRITE_SIZE / 2.0;
    let x_min = 0.0 + half_sprite_size;
    let y_min = 0.0 + half_sprite_size;
    let x_max = window.width() - half_sprite_size;
    let y_max = window.height() - half_sprite_size;
    for (mut position, mut velocity, mut acceleration) in boid_query.iter_mut() {
        if position.position.x > x_max {
            position.position.x = x_max;
            //velocity.velocity *= -1.0;
            rebond(&mut velocity, Vec2::new(-1.0,0.0));
        } else if position.position.x < x_min {
            position.position.x = x_min;
            //velocity.velocity *= -1.0;
            rebond(&mut velocity, Vec2::new(1.0,0.0));
        }
        if position.position.y > y_max {
            position.position.y = y_max;
            //velocity.velocity *= -1.0;
            rebond(&mut velocity, Vec2::new(0.0,-1.0));
        } else if position.position.y < y_min {
            position.position.y = y_min;
            //velocity.velocity *= -1.0;
            rebond(&mut velocity, Vec2::new(0.0,1.0));
        }
    }
}

pub fn rebond(mut velocity: &mut Velocity, normal: Vec2) {
    let angle_max: f32 = 45.0 * (std::f32::consts::PI / 180.0);
    let dot_velocity_normal = velocity.velocity.dot(normal);
    let reflection: Vec2 = velocity.velocity - 2.0 * dot_velocity_normal * normal;
    let angle = reflection.y.atan2(reflection.x);
    let mut rng = rand::thread_rng();
    let variation: f32 = rng.gen_range(-angle_max..angle_max);
    let new_angle = angle + variation;
    let magnitude = reflection.length();
    let new_velocity = Vec2::new(magnitude * f32::cos(new_angle), magnitude * f32::sin(new_angle));
    velocity.velocity = new_velocity;
}