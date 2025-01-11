use bevy::prelude::*;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy_spatial::SpatialAccess;
use rand::prelude::*;
use std::f32::consts::PI;
use crate::boids_3d::components::*;
use crate::boids_3d::resources::*;
use crate::boids_3d::bundles::*;
use crate::boids_3d::events::*;
use crate::boids_3d::cone::Cone;
use std::sync::Mutex;
use crate::kd_tree_3d::components::*;
use crate::boids_2d::components::ObstacleTag;

pub const BOUNDS_SIZE: f32 = 350.0;

const GROUP_COLORS: [Color; 2] = [
    Color::rgb(0.9, 0.3, 0.3),
    Color::rgb(0.3, 0.3, 0.9)
];
const GROUP_EMISSIVE: [Color; 2] = [
    Color::rgba(0.5, 0.0, 0.0, 0.5),
    Color::rgba(0.0, 0.0, 0.5, 0.5)
];

pub fn spawn_boid_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    boid_settings: &BoidSettings3D
) {
    let mut rng = rand::thread_rng();
    let group = rng.gen_range(0..2);
    
    let random_pos = Vec3::new(
        rng.gen_range(-50.0..50.0),
        rng.gen_range(-50.0..50.0),
        rng.gen_range(-50.0..50.0)
    );
    
    let theta = rng.gen_range(0.0..2.0 * PI);
    let phi = rng.gen_range(0.0..PI);
    let initial_velocity = Vec3::new(
        f32::sin(phi) * f32::cos(theta),
        f32::sin(phi) * f32::sin(theta),
        f32::cos(phi)
    );

    commands.spawn(BoidBundle {
        boid: Boid { group },
        velocity: Velocity { velocity: initial_velocity },
        acceleration: Acceleration { acceleration: Vec3::ZERO },
        pbr_bundle: PbrBundle {
            mesh: meshes.add(Mesh::from(Cone {
                radius: boid_settings.size,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: GROUP_COLORS[group as usize],
                emissive: GROUP_EMISSIVE[group as usize],
                ..default()
            }),
            transform: Transform {
                translation: random_pos,
                scale: Vec3::splat(boid_settings.size * 2.0),
                ..default()
            },
            ..default()
        },
        mode_3d_marker: Mode3DMarker,
        tracked_by_kdtree: TrackedByKDTree3D
    });
}

pub fn spawn_boids(
    mut commands: Commands,
    boid_settings: Res<BoidSettings3D>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in 0..boid_settings.count {
        spawn_boid_entity(&mut commands, &mut meshes, &mut materials, &boid_settings);
    }
}

pub fn flocking(
    boid_query: Query<(Entity, &Transform, &Velocity, &Boid), With<Boid>>,
    event_writer: EventWriter<ApplyForceEvent>,
    boid_settings: Res<BoidSettings3D>,
    groups_targets: Res<GroupsTargets>,
    kd_tree: Res<NNTree3D>,
) {
    let cohesion_range = boid_settings.cohesion_range;
    let alignment_range = boid_settings.alignment_range;
    let separation_range = boid_settings.separation_range;

    let event_writer = Mutex::new(event_writer);

    boid_query.par_iter().for_each(|(entity, transform, velocity, boid)| {
        let position = transform.translation;
        let mut cohesion_neighbors: Vec<Vec3> = Vec::new();
        let mut repulsion_neighbors: Vec<(Vec3, f32)> = Vec::new();
        let mut alignment_neighbors: Vec<Vec3> = Vec::new();

        for (_, neighbor_entity) in kd_tree.within_distance(position, cohesion_range) {
            let neighbor_entity = neighbor_entity.unwrap();
            if neighbor_entity == entity { continue; }

            if let Ok((_, neighbor_transform, neighbor_velocity, _)) = boid_query.get(neighbor_entity) {
                let neighbor_pos = neighbor_transform.translation;
                if let Some(distance) = is_in_field_of_view(&position, &velocity.velocity, &neighbor_pos, &boid_settings.field_of_view) {
                    if distance < separation_range {
                        repulsion_neighbors.push((neighbor_pos, distance));
                    } else if distance < alignment_range {
                        alignment_neighbors.push(neighbor_velocity.velocity);
                    } else if distance < cohesion_range {
                        cohesion_neighbors.push(neighbor_pos);
                    }
                }
            }
        }

        let cohesion_force = cohesion(&position, &cohesion_neighbors, &boid_settings.cohesion_coeff);
        let separation_force = separation(&position, &repulsion_neighbors, &boid_settings.separation_coeff);
        let alignment_force = alignment(&velocity.velocity, &alignment_neighbors, &boid_settings.alignment_coeff);
        let target = groups_targets.targets[boid.group as usize];
        let attraction_force = attraction_to_target(&position, &target, &boid_settings.attraction_coeff);
        let total_force = cohesion_force + separation_force + alignment_force + attraction_force;
        let mut event_writer = event_writer.lock().unwrap();
        event_writer.send(ApplyForceEvent {
            entity: entity,
            force: total_force });
    });
}

fn is_in_field_of_view(position: &Vec3, velocity: &Vec3, other_pos: &Vec3, fov: &f32) -> Option<f32> {
    let to_other = *other_pos - *position;
    let distance = to_other.length();
    
    if distance <= 0.0 { return None; }
    
    let cos_fov = (fov / 2.0).to_radians().cos();
    
    let dot = velocity.dot(to_other);
    
    if dot >= cos_fov * velocity.length() * distance {
        Some(distance)
    } else {
        None
    }
}

pub fn cohesion(position: &Vec3, cohesion_neighbors: &Vec<Vec3>, cohesion_coeff: &f32) -> Vec3 {
    let mut cohesion_force: Vec3 = Vec3::ZERO;
    let nb_neighbors = cohesion_neighbors.len();
    if nb_neighbors == 0 {
        return Vec3::ZERO;
    }
    for other_position in cohesion_neighbors.iter() {
        cohesion_force += *other_position;
    }
    cohesion_force /= nb_neighbors as f32;
    cohesion_force = cohesion_force - *position;
    cohesion_force * *cohesion_coeff
}

pub fn separation(position: &Vec3, repulsion_neighbors: &Vec<(Vec3, f32)>, separation_coeff: &f32) -> Vec3 {
    let mut separation_force: Vec3 = Vec3::ZERO;
    for (other_position, distance) in repulsion_neighbors.iter() {
        separation_force += (*position - *other_position) / *distance;
    }
    separation_force * *separation_coeff
}

pub fn alignment(velocity: &Vec3, alignment_neighbors: &Vec<Vec3>, alignment_coeff: &f32) -> Vec3 {
    let mut alignment_force: Vec3 = Vec3::ZERO;
    let nb_neighbors = alignment_neighbors.len();
    if nb_neighbors == 0 {
        return Vec3::ZERO;
    }
    for other_velocity in alignment_neighbors.iter() {
        alignment_force += *other_velocity;
    }
    alignment_force /= nb_neighbors as f32;
    alignment_force = alignment_force - *velocity;
    alignment_force * *alignment_coeff
}

pub fn attraction_to_target(position: &Vec3, target: &Vec3, attraction_coeff: &f32) -> Vec3 {
    (*target - *position) * *attraction_coeff
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
    mut boid_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Boid>>,
    boid_settings: Res<BoidSettings3D>,
    time: Res<Time>
) {
    for (mut transform, mut velocity, mut acceleration) in boid_query.iter_mut() {
        velocity.velocity += acceleration.acceleration * time.delta_seconds();
        
        let speed = velocity.velocity.length();
        if speed < boid_settings.min_speed {
            velocity.velocity = velocity.velocity.normalize() * boid_settings.min_speed;
        } else if speed > boid_settings.max_speed {
            velocity.velocity = velocity.velocity.normalize() * boid_settings.max_speed;
        }

        transform.translation += velocity.velocity * time.delta_seconds();
        
        if velocity.velocity.length_squared() > 0.0 {
            let forward = -velocity.velocity.normalize();
            transform.rotation = Quat::from_rotation_arc(Vec3::Z, forward);
        }

        acceleration.acceleration = Vec3::ZERO;
    }
}

pub fn confine_movement (
    mut boid_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Boid>>,
    boid_settings: Res<BoidSettings3D>
) {
    let margin = BOUNDS_SIZE * 0.2;
    let x_min = -BOUNDS_SIZE + margin;
    let y_min = -BOUNDS_SIZE + margin;
    let z_min = -BOUNDS_SIZE + margin;
    let x_max = BOUNDS_SIZE - margin;
    let y_max = BOUNDS_SIZE - margin;
    let z_max = BOUNDS_SIZE - margin;
    for (mut transform, mut velocity, _) in boid_query.iter_mut() {
        if boid_settings.bounce_against_walls {
            let turn_factor: f32 = 10.0;
            if transform.translation.x > x_max - margin {
                velocity.velocity.x -= turn_factor;
            }
            if transform.translation.x < x_min + margin {
                velocity.velocity.x += turn_factor;
            }
            if transform.translation.y > y_max - margin {
                velocity.velocity.y -= turn_factor;
            }
            if transform.translation.y < y_min + margin {
                velocity.velocity.y += turn_factor;
            }
            if transform.translation.z > z_max - margin {
                velocity.velocity.z -= turn_factor;
            }
            if transform.translation.z < z_min + margin {
                velocity.velocity.z += turn_factor;
            }
        }
        else {
            if transform.translation.x > x_max {
                transform.translation.x = x_min;
            } else if transform.translation.x < x_min {
                transform.translation.x = x_max;
            }
            if transform.translation.y > y_max {
                transform.translation.y = y_min;
            } else if transform.translation.y < y_min {
                transform.translation.y = y_max;
            }
            if transform.translation.z > z_max {
                transform.translation.z = z_min;
            } else if transform.translation.z < z_min {
                transform.translation.z = z_max;
            }
        }
    }
}

pub fn adjust_population(
    boid_query: Query<Entity, With<Boid>>,
    mut commands: Commands,
    mut boid_settings: ResMut<BoidSettings3D>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let current_count = boid_settings.count;
    let previous_count = boid_settings.previous_count;

    if current_count == previous_count {
        return;
    } 
    else if current_count > previous_count {
        for _ in 0..(current_count - previous_count) {
            spawn_boid_entity(&mut commands, &mut meshes, &mut materials, &boid_settings);
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

pub fn resize_boids(
    mut boid_query: Query<&mut Transform, With<Boid>>,
    mut resize_event_reader: EventReader<ResizeEvent>
) {
    if let Some(ev) = resize_event_reader.read().last() {
        for mut transform in boid_query.iter_mut() {
            transform.scale = Vec3::splat(ev.scale);
        }
    }
}

pub fn spawn_obstacle_3d(
    commands: &mut Commands,
    position: Vec3,
    color: Vec3,
    radius: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::UVSphere {
        radius, 
        sectors: 32, 
        stacks: 16, 
    }));
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(color.x, color.y, color.z),
        ..default()
    });

    commands.spawn((
        ObstacleBundle {
            pbr_bundle: PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(position),
                ..default()
            },
        },
        ObstacleTag,
         ));
}

pub fn setup_3d_scene(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10000.0,
                ..default()
            },
            transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Mode3DMarker
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    let boundary_material = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 0.5, 0.0, 0.05),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(
            BOUNDS_SIZE * 2.0,
            BOUNDS_SIZE * 2.0,
            BOUNDS_SIZE * 2.0,
        ))),
        material: boundary_material,
        transform: Transform::from_xyz(0.0, 0.1, 0.0),
        ..default()
    },
    Mode3DMarker,
    NotShadowCaster,
    NotShadowReceiver
    ));

    let ground_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(BOUNDS_SIZE * 4.0))),
            material: ground_material,
            transform: Transform::from_xyz(0.0, -BOUNDS_SIZE, 0.0),
            ..default()
        },
        Mode3DMarker));
}