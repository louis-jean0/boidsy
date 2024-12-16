use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use bevy::transform;
use bevy::window::PrimaryWindow;
use bevy_spatial::SpatialAccess;
use rand::prelude::*;
use crate::boids_2d::components::*;
use crate::boids_2d::resources::*;
use crate::boids_2d::bundles::*;
use crate::boids_2d::events::*;
<<<<<<< HEAD
use crate::kd_tree_2d::components::*;
=======

use bevy::sprite::MaterialMesh2dBundle;

>>>>>>> Ben

pub const SPRITE_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct ObstacleTag;


pub fn spawn_boid_entity(
    commands: &mut Commands,
    window: &Window,
    asset_server: &Res<AssetServer>
) {
    let texture_path = "../assets/fish.png";
    let mut rng = rand::thread_rng();
    let random_x: f32 = rng.gen_range(0.0..window.width());
    let random_y: f32 = rng.gen_range(0.0..window.height());
    let random_group: u8 = rng.gen_range(0..2);
    let random_angle: f32 = rng.gen_range(0.0..1.0) * 360.0 * (std::f32::consts::PI / 180.0); // En radians
    commands.spawn(
        BoidBundle {
            boid: Boid {
                group: random_group
            },
            // transform: Transform {
            //     translation: Vec3::new(random_x, random_y, 0.0),
            //     ..default()
            // },
            velocity: Velocity {
                velocity: Vec2::new(f32::cos(random_angle), f32::sin(random_angle))
            },
            acceleration: Acceleration {
                acceleration: Vec2::new(0.0,0.0)
            },
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(random_x, random_y, 0.0),
                    ..default()
                },
                texture: asset_server.load(texture_path),
                ..default()
            },
            tracked_by_kdtree: TrackedByKDTree
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
        spawn_boid_entity(&mut commands, &window, &asset_server);
    }
}

pub fn flocking(
    boid_query: Query<(Entity, &Transform, &Velocity, &Boid)>,
    mut event_writer: EventWriter<ApplyForceEvent>,
    boid_settings: Res<BoidSettings>,
    groups_targets: Res<GroupsTargets>,
    kd_tree: Res<NNTree>
) {
    let cohesion_range = boid_settings.cohesion_range;
    let alignment_range = boid_settings.alignment_range;
    let separation_range = boid_settings.separation_range;

    for(entity, transform, velocity, boid) in boid_query.iter() {
        let position = transform.translation.truncate();
        let mut cohesion_neighbors: Vec<Vec2> = Vec::new();
        let mut repulsion_neighbors: Vec<(Vec2, f32)> = Vec::new();
        let mut alignment_neighbors: Vec<Vec2> = Vec::new();
        for (_, neighbor_entity) in kd_tree.within_distance(position, cohesion_range) {
            if let Some(neighbor_entity) = neighbor_entity {
                if let Ok((_, neighbor_transform, neighbor_velocity, _)) = boid_query.get(neighbor_entity) {
                    let neighbor_position = neighbor_transform.translation.truncate();
                    let distance = position.distance(neighbor_position);
                    if distance < separation_range {
                        repulsion_neighbors.push((neighbor_position, distance));
                    } else if distance < alignment_range {
                        alignment_neighbors.push(neighbor_velocity.velocity);
                    } else if distance < cohesion_range {
                        cohesion_neighbors.push(neighbor_position);
                    }
                } else {
                    warn!("Could not fetch components for entity {:?}", neighbor_entity);
                }
            }
        }  
        let cohesion_force: Vec2 = cohesion(&position, &cohesion_neighbors, &boid_settings.cohesion_coeff);
        let avoidance_force: Vec2 = avoidance(&position, &repulsion_neighbors, &boid_settings.separation_coeff, &boid_settings.min_distance_between_boids, &boid_settings.collision_coeff);
        let alignment_force: Vec2 = alignment(&velocity, &alignment_neighbors, &boid_settings.alignment_coeff);
        let target = groups_targets.targets[boid.group as usize];
        let attraction_force: Vec2 = attraction_to_target(&position, &target, &boid_settings.attraction_coeff);
        let total_force = cohesion_force + avoidance_force + alignment_force + attraction_force;
        event_writer.send(ApplyForceEvent {
            entity: entity,
            force: total_force
        });
    }
}

// pub fn get_neighbors_in_radius<'a>(
//     position: &Position,
//     cohesion_range: f32,
//     kd_tree: Res<NNTree>,
//     boid_query: Query<(Entity, &'a Position, &'a Velocity), With<Boid>>) -> Vec<(Entity, &'a Position, &'a Velocity)>
// {

//     let results = kd_tree.within_distance(position.position, cohesion_range);
//     results.into_iter()
//     .filter_map(|result| {

//     })
// }

pub fn cohesion(position: &Vec2, cohesion_neighbors: &Vec<Vec2>, cohesion_coeff: &f32) -> Vec2 {
    let mut cohesion_force: Vec2 = Vec2::ZERO;
    let nb_neighbors = cohesion_neighbors.len();
    if nb_neighbors == 0 {
        return Vec2::ZERO;
    }
    for other_position in cohesion_neighbors.iter() {
        cohesion_force += *other_position;
    }
    cohesion_force /= nb_neighbors as f32;
    cohesion_force = cohesion_force - *position;
    cohesion_force * *cohesion_coeff
}

pub fn avoidance(position: &Vec2, repulsion_neighbors: &Vec<(Vec2, f32)>, separation_coeff: &f32, min_distance_between_boids: &f32, collision_coeff: &f32) -> Vec2 {
    let mut avoidance_force: Vec2 = Vec2::ZERO;
    for (other_position, distance) in repulsion_neighbors.iter() {
        if distance < min_distance_between_boids {
            let interpolation_factor = (*min_distance_between_boids - distance) / *min_distance_between_boids;
            avoidance_force += (*position - *other_position) * *collision_coeff * interpolation_factor;
        }
        else {
            avoidance_force += *position - *other_position;
        }
    }
    avoidance_force * *separation_coeff
}

pub fn alignment(velocity: &Velocity, alignment_neighbors: &Vec<Vec2>, alignment_coeff: &f32) -> Vec2 {
    let mut alignment_force: Vec2 = Vec2::ZERO;
    let nb_neighbors = alignment_neighbors.len();
    if nb_neighbors == 0 {
        return Vec2::ZERO;
    }
    for other_velocity in alignment_neighbors.iter() {
        alignment_force += *other_velocity;
    }
    alignment_force /= nb_neighbors as f32;
    alignment_force = alignment_force - velocity.velocity;
    alignment_force * *alignment_coeff
}

pub fn attraction_to_target(position: &Vec2, target: &Vec2, attraction_coeff: &f32) -> Vec2 {
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
    boid_settings: Res<BoidSettings>,
    time: Res<Time>
) {
    for(mut transform, mut velocity, mut acceleration) in boid_query.iter_mut() {
        velocity.velocity += acceleration.acceleration * time.delta_seconds();
        if velocity.velocity.length() < boid_settings.min_speed {
            velocity.velocity = velocity.velocity.normalize() * boid_settings.min_speed;
        }
        if velocity.velocity.length() > boid_settings.max_speed {
            velocity.velocity = velocity.velocity.normalize() * boid_settings.max_speed;
        }
        transform.translation += Vec3::new(velocity.velocity.x, velocity.velocity.y, 0.0) * time.delta_seconds();
        acceleration.acceleration = Vec2::ZERO;
        let rotation_angle = velocity.velocity.y.atan2(velocity.velocity.x);
        transform.rotation = Quat::from_rotation_z(rotation_angle);
    }
}

pub fn is_in_field_of_view(position: &Vec2, velocity: &Velocity, other_position: &Vec2, fov: &f32) -> bool {
    let to_other = *other_position - *position;
    let distance = to_other.length();
    false
}

pub fn confine_movement (
    mut boid_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Boid>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    boid_settings: Res<BoidSettings>
) {
    let window = window_query.get_single().unwrap();
    let half_sprite_size = SPRITE_SIZE / 2.0;
    let x_min = 0.0 + half_sprite_size;
    let y_min = 0.0 + half_sprite_size;
    let x_max = window.width() - half_sprite_size;
    let y_max = window.height() - half_sprite_size;
    for (mut transform, mut velocity, _) in boid_query.iter_mut() {
        if boid_settings.bounce_against_walls {
            let turn_factor: f32 = 20.0;
            let margin = 100.0;
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
        }
    }
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
            spawn_boid_entity(&mut commands, &window, &asset_server);
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

pub fn spawn_obstacle(
    commands: &mut Commands,
    position: Vec2,
    color: Vec3,
    radius: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Circle::new(radius)));
            // meshes.add(CircularSector::new(50.0, 1.0)),
        // meshes.add(CircularSegment::new(50.0, 1.25)),
        // meshes.add(Ellipse::new(25.0, 50.0)),
        // meshes.add(Annulus::new(25.0, 50.0)),
        // meshes.add(Capsule2d::new(25.0, 50.0)),
        // meshes.add(Rhombus::new(75.0, 100.0)),
        // meshes.add(Rectangle::new(50.0, 100.0)),
        // meshes.add(RegularPolygon::new(50.0, 6)),
        // meshes.add(Triangle2d::new(
        //     Vec2::Y * 50.0,
        //     Vec2::new(-50.0, -50.0),
        //     Vec2::new(50.0, -50.0),
        // )),
    let material = materials.add(Color::rgb(color.x, color.y, color.z).into());

    commands.spawn((
        ObstacleBundle {
<<<<<<< HEAD
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 0.0),
                ..default()
            }
        },
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 0.0),
                scale: Vec3::splat(size / SPRITE_SIZE), // Ajuste la taille en fonction du rayon
=======
            position: Position { position },
            material_mesh: MaterialMesh2dBundle {
                mesh: mesh.into(),
                material,
                transform: Transform::from_xyz(position.x, position.y, 1.0),
>>>>>>> Ben
                ..default()
            },
        },
        ObstacleTag, // Ajout du tag pour marquer l'entité comme obstacle
    ));
}


pub fn remove_all_obstacles(mut commands: Commands, query: Query<Entity, With<ObstacleTag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}





pub fn spawn_obstacles_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
<<<<<<< HEAD
    // Exemple : Créer un obstacle à la position (200, 300) avec une taille de 50
    //spawn_obstacle(&mut commands, Vec2::new(200.0, 300.0), 50.0, &asset_server);
=======
    // Exemple : Créer un obstacle à la position (200, 300) avec un rayon de 50
    spawn_obstacle(&mut commands, Vec2::new(200.0, 300.0), Vec3::new(1.0, 0., 0.), 10.0, &mut meshes, &mut materials);
    spawn_obstacle(&mut commands, Vec2::new(500.0, 750.0), Vec3::new(0., 1., 0.), 32.0, &mut meshes, &mut materials);
    spawn_obstacle(&mut commands, Vec2::new(900.0, 100.0), Vec3::new(0., 0., 1.), 50.0, &mut meshes, &mut materials);
>>>>>>> Ben
}


