use bevy::prelude::*;
use bevy_spatial::SpatialAccess;
use rand::prelude::*;
use crate::boids_2d::components::ObstacleTag;
use crate::underwater::terrain::{GROUND_Y_POSITION, TERRAIN_SCALE, TERRAIN_SIZE};
use crate::underwater::UnderwaterMarker;
use crate::boids_3d::{bundles::BoidBundle, components::*, events::ApplyForceEvent};
use crate::kd_tree_3d::components::{NNTree3D, TrackedByKDTree3D};
use super::components::*;
use crate::boids_3d::systems::*;

pub fn load_fish_models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FishModels {
        small_fish: asset_server.load("models/clown_fish/scene.gltf#Scene0"),
        medium_fish: asset_server.load("models/koi_fish/scene.gltf#Scene0"),
        large_fish: asset_server.load("models/puffer_fish/scene.gltf#Scene0")
    });
}

pub fn spawn_fish_schools(
    mut commands: Commands,
    fish_models: Res<FishModels>,
) {
    let mut rng = rand::thread_rng();
    
    for species in [Species::SmallFish, Species::MediumFish, Species::LargeFish] {
        let settings = species.get_settings();
        let num_schools = match species {
            Species::SmallFish => 10,
            Species::MediumFish => 5,
            Species::LargeFish => 3,
        };

        for school_id in 0..num_schools {
            let school_center = Vec3::new(
                rng.gen_range(-TERRAIN_SIZE/2.0..TERRAIN_SIZE/2.0),
                rng.gen_range(GROUND_Y_POSITION..-10.0),
                rng.gen_range(-TERRAIN_SIZE/2.0..TERRAIN_SIZE/2.0),
            );

            spawn_fish_school(
                &mut commands,
                &fish_models,
                &species,
                school_id,
                school_center,
                &settings,
            );
        }
    }
}

fn spawn_fish_school(
    commands: &mut Commands,
    models: &FishModels,
    species: &Species,
    school_id: usize,
    center: Vec3,
    settings: &UnderwaterBoidSettings,
) {
    let mut rng = rand::thread_rng();
    let model = match species {
        Species::SmallFish => &models.small_fish,
        Species::MediumFish => &models.medium_fish,
        Species::LargeFish => &models.large_fish,
    };

    for _ in 0..settings.count {
        let offset = Vec3::new(
            rng.gen_range(-20.0..20.0),
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-20.0..20.0),
        );

        commands.spawn((
            BoidBundle {
                boid: Boid { group: school_id as u8 },
                velocity: Velocity { 
                    velocity: Vec3::new(
                        rng.gen_range(-1.0..1.0),
                        rng.gen_range(-0.5..0.5),
                        rng.gen_range(-1.0..1.0),
                    ).normalize() * settings.min_speed
                },
                acceleration: Acceleration {
                    acceleration: Vec3::ZERO
                },
                pbr_bundle: PbrBundle {
                    transform: Transform::from_translation(center + offset)
                        .with_scale(Vec3::splat(settings.size)),
                    ..default()
                },
                tracked_by_kdtree: TrackedByKDTree3D,
            },
            FishType {
                species: species.clone(),
                school_id,
            },
            UnderwaterMarker,
        ))
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: model.clone(),
                transform: Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
                ..default()
            });
        });
    }
}

pub fn apply_underwater_flocking(
    boid_query: Query<(Entity, &Transform, &Velocity, &Boid, &FishType), With<UnderwaterMarker>>,
    event_writer: EventWriter<ApplyForceEvent>,
    kd_tree: Res<NNTree3D>
) {
    let event_writer = std::sync::Mutex::new(event_writer);

    boid_query.par_iter().for_each(|(entity, transform, velocity, _, fish_type)| {
        let position = transform.translation;
        let settings = fish_type.species.get_settings();
        
        let mut cohesion_neighbors = Vec::new();
        let mut repulsion_neighbors = Vec::new();
        let mut alignment_neighbors = Vec::new();

        let max_range = settings.cohesion_range.max(settings.alignment_range).max(settings.separation_range);

        for (_, neighbor_entity) in kd_tree.within_distance(position, max_range) {
            let neighbor_entity = neighbor_entity.unwrap();
            if neighbor_entity == entity { continue; }

            if let Ok((_, neighbor_transform, neighbor_velocity, _, neighbor_fish_type)) = boid_query.get(neighbor_entity) {
                if std::mem::discriminant(&fish_type.species) != std::mem::discriminant(&neighbor_fish_type.species) {
                    continue;
                }

                let neighbor_pos = neighbor_transform.translation;
                if let Some(distance) = is_in_field_of_view(&position, &velocity.velocity, &neighbor_pos, &settings.field_of_view) {
                    if distance < settings.separation_range {
                        repulsion_neighbors.push((neighbor_pos, distance));
                    } else if distance < settings.alignment_range {
                        alignment_neighbors.push(neighbor_velocity.velocity);
                    } else if distance < settings.cohesion_range {
                        cohesion_neighbors.push(neighbor_pos);
                    }
                }
            }
        }

        let cohesion_force = cohesion(&position, &cohesion_neighbors, &settings.cohesion_coeff);
        let separation_force = separation(&position, &repulsion_neighbors, &settings.separation_coeff);
        let alignment_force = alignment(&velocity.velocity, &alignment_neighbors, &settings.alignment_coeff);
        
        let total_force = cohesion_force + separation_force + alignment_force;

        let mut event_writer = event_writer.lock().unwrap();
        event_writer.send(ApplyForceEvent {
            entity,
            force: total_force 
        });
    });
}

pub fn update_fish_positions(
    mut boid_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Boid>>,
    boid_settings: Res<UnderwaterBoidSettings>,
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

pub fn avoid_obstacles(
    mut boid_query: Query<(Entity, &Transform, &mut Velocity), With<Boid>>,
    mut event_writer: EventWriter<ApplyForceEvent>,
    obstacles_query: Query<(&Transform, &ObstacleTag)>) {
    for (entity, transform, mut velocity) in boid_query.iter_mut() {
        let position = transform.translation;
        let mut avoidance_force: Vec3 = Vec3::ZERO;
        let obstacle_avoidance_range = 50.0;
        let obstacle_avoidance_coeff: f32 = 10.0;
        let turn_factor: f32 = 20.0;
        for (obstacle_transform, _) in obstacles_query.iter() {
            let obstacle_position = obstacle_transform.translation;
            let distance = position.distance(obstacle_position) - 10.0;
            if distance < obstacle_avoidance_range {
                let direction = (position - obstacle_position).normalize();
                let interpolation_factor = (obstacle_avoidance_range - distance) / obstacle_avoidance_range;
                let force_magnitude = obstacle_avoidance_coeff * interpolation_factor;
                avoidance_force += direction * force_magnitude;
                velocity.velocity += direction * turn_factor * interpolation_factor;
            }
        }
        event_writer.send(ApplyForceEvent {
            entity: entity,
            force: avoidance_force
        });
    }
}

pub fn confine_fishes_movement (
    mut boid_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Boid>>
) {
    let margin = TERRAIN_SCALE * 0.2;
    let x_min = -200.0 + margin;
    let y_min = GROUND_Y_POSITION;
    let z_min = -200.0 + margin;
    let x_max = 200.0 - margin;
    let y_max = 30.0;
    let z_max = 200.0 - margin;
    for (transform, mut velocity, _) in boid_query.iter_mut() {
        let turn_factor: f32 = 100.0;
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
}