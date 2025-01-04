use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_spatial::SpatialAccess;
use rand::prelude::*;
use std::sync::Mutex;
use crate::boids_2d::components::*;
use crate::boids_2d::resources::*;
use crate::boids_2d::bundles::*;
use crate::boids_2d::events::*;

use bevy::sprite::MaterialMesh2dBundle;
use crate::kd_tree_2d::components::*;
use crate::input::resources::ShapeSettings;
use crate::input::systems::*;
use crate::ui::events::CursorVisibilityEvent;

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
    boid_query: Query<(Entity, &Transform, &Velocity, &Boid), With<Boid>>,
    event_writer: EventWriter<ApplyForceEvent>,
    boid_settings: Res<BoidSettings>,
    groups_targets: Res<GroupsTargets>,
    kd_tree: Res<NNTree>
) {
    let cohesion_range = boid_settings.cohesion_range;
    let alignment_range = boid_settings.alignment_range;
    let separation_range = boid_settings.separation_range;

    let event_writer = Mutex::new(event_writer);

    boid_query.par_iter().for_each(|(entity, transform, velocity, boid)| {
        let position = transform.translation.truncate();
        let mut cohesion_neighbors: Vec<Vec2> = Vec::new();
        let mut repulsion_neighbors: Vec<(Vec2, f32)> = Vec::new();
        let mut alignment_neighbors: Vec<Vec2> = Vec::new();
        for (_, neighbor_entity) in kd_tree.within_distance(position, cohesion_range) {
            if let Some(neighbor_entity) = neighbor_entity {
                if let Ok((_, neighbor_transform, neighbor_velocity, _)) = boid_query.get(neighbor_entity) {
                    let neighbor_position = neighbor_transform.translation.truncate();
                    if let Some(distance) = is_in_field_of_view(&position, &neighbor_position, &boid_settings.field_of_view, &boid_settings.cohesion_range) {
                        if distance < separation_range {
                            repulsion_neighbors.push((neighbor_position, distance));
                        } else if distance < alignment_range {
                            alignment_neighbors.push(neighbor_velocity.velocity);
                        } else if distance < cohesion_range {
                            cohesion_neighbors.push(neighbor_position);
                        }
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
        let mut event_writer = event_writer.lock().unwrap();
        event_writer.send(ApplyForceEvent {
            entity: entity,
            force: total_force
        });
    });
}

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

pub fn avoid_obstacles(
    mut boid_query: Query<(Entity, &Transform, &mut Velocity), With<Boid>>,
    mut event_writer: EventWriter<ApplyForceEvent>,
    obstacles_query: Query<(&Transform, &ObstacleTag)>,
    shape_settings: Res<ShapeSettings>
) {
    for (entity, transform, mut velocity) in boid_query.iter_mut() {
        let position = transform.translation.truncate();
        let mut avoidance_force: Vec2 = Vec2::ZERO;
        let obstacle_avoidance_range = 50.0;
        let obstacle_avoidance_coeff = 10.0;
        let turn_factor: f32 = 20.0;
        for (obstacle_transform, _) in obstacles_query.iter() {
            let obstacle_position = obstacle_transform.translation.truncate();
            let distance = position.distance(obstacle_position) - shape_settings.radius;
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

pub fn scare_with_cursor(
    mut commands: Commands,
    mut boid_query: Query<(Entity, &Transform, &mut Velocity), With<Boid>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut event_writer: EventWriter<ApplyForceEvent>,
    mut cursor_visibility_writer: EventWriter<CursorVisibilityEvent>,
    mouse_button_input: Res<Input<MouseButton>>,
    kd_tree: Res<NNTree>,
    asset_server: Res<AssetServer>,
    shark_query: Query<Entity, With<Shark>>
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        if let Some(cursor_pos) = cursor_position(&window_query) {
            cursor_visibility_writer.send(CursorVisibilityEvent {
                visible: false
            });
            let fear_range = 100.0;
            let cursor_radius = 50.0;
            let fear_coeff = 100000.0;
            let turn_factor: f32 = 20.0;

            let shark_texture = asset_server.load("../assets/shark.png");
            commands.spawn((
                SpriteBundle {
                    texture: shark_texture,
                    transform: Transform {
                        translation: Vec3::new(cursor_pos.x, cursor_pos.y, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Shark
            ));

            let nearby_boids = kd_tree.within_distance(cursor_pos, fear_range + cursor_radius);

            for (_, entity) in nearby_boids {
                if let Some(entity) = entity {
                    if let Ok((_, transform, mut velocity)) = boid_query.get_mut(entity) {
                        let position = transform.translation.truncate();
                        let distance = position.distance(cursor_pos) - cursor_radius;
                        let direction = (position - cursor_pos).normalize();
                        let interpolation_factor = (fear_range - distance) / fear_range;
                        let force_magnitude = fear_coeff * interpolation_factor;
                        let avoidance_force = direction * force_magnitude;
                        velocity.velocity += direction * turn_factor * interpolation_factor;
                        event_writer.send(ApplyForceEvent {
                            entity: entity,
                            force: avoidance_force,
                        });
                    }
                }
            }
        }
        for shark in shark_query.iter() {
            commands.entity(shark).despawn();
        }
    }
    else {
        cursor_visibility_writer.send(CursorVisibilityEvent {
            visible: true
        });
        for shark in shark_query.iter() {
            commands.entity(shark).despawn();
        }
    }
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

fn is_in_field_of_view(position: &Vec2, other_position: &Vec2, fov: &f32, cohesion_range: &f32) -> Option<f32> {
    let to_other = *other_position - *position;
    let distance_squared = to_other.length_squared();
    let cohesion_range_squared = cohesion_range * cohesion_range;
    if distance_squared < cohesion_range_squared {
        let angle_between_the_two = position.angle_between(*other_position) * (180.0 / std::f32::consts::PI);
        if angle_between_the_two <= *fov {
            return Some(distance_squared.sqrt());
        }
    }
    None
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
        //meshes.add(Mesh::from(shape::Quad::new(Vec2::new(radius, 0.0))));
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
            material_mesh: MaterialMesh2dBundle {
                mesh: mesh.into(),
                material,
                transform: Transform::from_xyz(position.x, position.y, 1.0),
                ..Default::default()
            },
        },
        ObstacleTag,
    ));
}

pub fn remove_all_obstacles(mut commands: Commands, query: Query<Entity, With<ObstacleTag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}