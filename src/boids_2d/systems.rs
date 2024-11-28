use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use crate::boids_2d::components::*;
use crate::boids_2d::resources::*;
use crate::boids_2d::bundles::*;
use crate::boids_2d::events::*;

pub const SPRITE_SIZE: f32 = 32.0;

pub fn spawn_boid_entity(
    commands: &mut Commands,
    window: &Window,
    asset_server: &Res<AssetServer>,
    boid_type: BoidType
) {
    let texture_path = match boid_type {
        BoidType::Bird => "../assets/bird.png",
        BoidType::Fish => "../assets/fish.png"
    };

    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();
    let random_angle = random::<f32>() * 360.0 * (std::f32::consts::PI / 180.0); // En radians
    commands.spawn(
        BoidBundle {
            boid: Boid {
                boid_type: boid_type
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

pub fn spawn_boids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    boid_settings: Res<BoidSettings>) {

    let window = window_query.get_single().unwrap();
    for _ in 0..boid_settings.count {
        spawn_boid_entity(&mut commands, &window, &asset_server, boid_settings.boid_type);
    }
}

pub fn flocking(
    boid_query: Query<(Entity, &Position, &Velocity), With<Boid>>,
    mut event_writer: EventWriter<ApplyForceEvent>,
    boid_settings: Res<BoidSettings>
) {
    let cohesion_range = boid_settings.cohesion_range;
    let alignment_range = boid_settings.alignment_range;
    let separation_range = boid_settings.separation_range;

    for(entity, position, velocity) in boid_query.iter() {
        let mut cohesion_neighbors: Vec<&Position> = Vec::new();
        let mut repulsion_neighbors: Vec<(&Position, f32)> = Vec::new();
        let mut alignment_neighbors: Vec<&Velocity> = Vec::new();
        for (other_entity, other_position, other_velocity) in boid_query.iter() {
            if entity == other_entity {
                continue;
            }
            let distance = position.position.abs().distance(other_position.position.abs());
            if distance < separation_range {
                repulsion_neighbors.push((other_position, distance));
            }
            else if distance < cohesion_range {
                cohesion_neighbors.push(other_position);
                
            }
            else if distance < alignment_range {
                alignment_neighbors.push(other_velocity);
            }
        }
        let cohesion_force: Vec2 = cohesion(position, &cohesion_neighbors, &boid_settings.cohesion_coeff);
        let avoidance_force: Vec2 = avoidance(position, &repulsion_neighbors, &boid_settings.separation_coeff, &boid_settings.min_distance_between_boids, &boid_settings.collision_coeff);
        let alignment_force: Vec2 = alignment(&velocity, &alignment_neighbors, &boid_settings.alignment_coeff);
        let total_force = cohesion_force + avoidance_force + alignment_force;
        event_writer.send(ApplyForceEvent {
            entity: entity,
            force: total_force
        });
    }
}

pub fn cohesion(position: &Position, cohesion_neighbors: &Vec<&Position>, cohesion_coeff: &f32) -> Vec2 {
    let mut cohesion_force: Vec2 = Vec2::ZERO;
    let nb_neighbors = cohesion_neighbors.len();
    if nb_neighbors == 0 {
        return Vec2::ZERO;
    }
    for other_position in cohesion_neighbors.iter() {
        cohesion_force += other_position.position;
    }
    cohesion_force /= nb_neighbors as f32;
    cohesion_force = cohesion_force - position.position;
    cohesion_force * *cohesion_coeff
}

pub fn avoidance(position: &Position, repulsion_neighbors: &Vec<(&Position, f32)>, separation_coeff: &f32, min_distance_between_boids: &f32, collision_coeff: &f32) -> Vec2 {
    let mut avoidance_force: Vec2 = Vec2::ZERO;
    for (other_position, distance) in repulsion_neighbors.iter() {
        if distance < min_distance_between_boids {
            let interpolation_factor = (*min_distance_between_boids - distance) / *min_distance_between_boids;
            avoidance_force += (position.position - other_position.position) * *collision_coeff * interpolation_factor;
        }
        else {
            avoidance_force += position.position - other_position.position;
        }
    }
    avoidance_force * *separation_coeff
}

pub fn alignment(velocity: &Velocity, alignment_neighbors: &Vec<&Velocity>, alignment_coeff: &f32) -> Vec2 {
    let mut alignment_force: Vec2 = Vec2::ZERO;
    let nb_neighbors = alignment_neighbors.len();
    if nb_neighbors == 0 {
        return Vec2::ZERO;
    }
    for other_velocity in alignment_neighbors.iter() {
        alignment_force += other_velocity.velocity;
    }
    alignment_force /= nb_neighbors as f32;
    alignment_force = alignment_force - velocity.velocity;
    alignment_force * *alignment_coeff
}

pub fn apply_forces_system(
    mut forces: EventReader<ApplyForceEvent>,
    mut boid_query: Query<&mut Acceleration, With<Boid>>
) {
    for ApplyForceEvent{entity, force} in forces.read() {
        if let Ok(mut acceleration) = boid_query.get_mut(*entity) {
            acceleration.acceleration += *force;
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
        let rotation_angle = velocity.velocity.y.atan2(velocity.velocity.x);
        transform.rotation = Quat::from_rotation_z(rotation_angle);
    }
}

pub fn confine_movement (
    mut boid_query: Query<(&mut Position, &mut Velocity, &mut Acceleration), With<Boid>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    boid_settings: Res<BoidSettings>
) {
    let window = window_query.get_single().unwrap();
    let half_sprite_size = SPRITE_SIZE / 2.0;
    let x_min = 0.0 + half_sprite_size;
    let y_min = 0.0 + half_sprite_size;
    let x_max = window.width() - half_sprite_size;
    let y_max = window.height() - half_sprite_size;
    for (mut position, mut velocity, _) in boid_query.iter_mut() {
        if boid_settings.bounce_against_walls {
            if position.position.x > x_max {
                position.position.x = x_max;
                rebond(&mut velocity, Vec2::new(-1.0,0.0));
            } else if position.position.x < x_min {
                position.position.x = x_min;
                rebond(&mut velocity, Vec2::new(1.0,0.0));
            }
            if position.position.y > y_max {
                position.position.y = y_max;
                rebond(&mut velocity, Vec2::new(0.0,-1.0));
            } else if position.position.y < y_min {
                position.position.y = y_min;
                rebond(&mut velocity, Vec2::new(0.0,1.0));
            }
        }
        else {
            if position.position.x > x_max {
                position.position.x = x_min;
            } else if position.position.x < x_min {
                position.position.x = x_max;
            }
            if position.position.y > y_max {
                position.position.y = y_min;
            } else if position.position.y < y_min {
                position.position.y = y_max;
            }
        }
    }
}

pub fn rebond(velocity: &mut Velocity, normal: Vec2) {
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

pub fn adjust_population(
    boid_query: Query<Entity, With<Boid>>,
    mut commands: Commands,
    mut boid_settings: ResMut<BoidSettings>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let current_count = boid_settings.count;
    let previous_count = boid_settings.previous_count;
    let window = window_query.get_single().unwrap();

    if current_count == previous_count {
        return;
    }
    else if current_count > previous_count {
        for _ in 0..(current_count - previous_count) {
            spawn_boid_entity(&mut commands, &window, &asset_server, boid_settings.boid_type);
        }
    }
    else {
        let to_remove = previous_count - current_count;
        for entity in boid_query.iter().take(to_remove) {
            commands.entity(entity).despawn();
        }
    }
    boid_settings.previous_count = current_count;
}