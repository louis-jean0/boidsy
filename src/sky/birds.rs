use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_spatial::SpatialAccess;
use rand::Rng;
use crate::boids_2d::components::ObstacleTag;
use crate::boids_3d::events::ApplyForceEvent;
use crate::boids_3d::resources::GroupsTargets;
use crate::boids_3d::{
    bundles::BoidBundle,
    components::*,
    resources::BoidSettings3D,
    systems::*,
};
use crate::kd_tree_3d::components::{NNTree3D, TrackedByKDTree3D};
use crate::ui::resources::SimulationState;
use super::SkySceneMarker;
use std::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct BirdModel(Handle<Scene>);

#[derive(Resource)]
pub struct SkyBoidSettings(BoidSettings3D);

impl Default for SkyBoidSettings {
    fn default() -> Self {
        Self(BoidSettings3D {
            count: 1000,
            previous_count: 1000,
            size: 1.5,
            cohesion_range: 50.0,
            alignment_range: 30.0,
            separation_range: 20.0,
            min_distance_between_boids: 50.0,
            cohesion_coeff: 20.0,
            alignment_coeff: 5.0,
            separation_coeff: 20.0,
            collision_coeff: 24.0,
            min_speed: 50.0,
            max_speed: 150.0,
            bounce_against_walls: true,
            attraction_coeff: 1.0,
            field_of_view: 90.0,
        })
    }
}

impl Deref for SkyBoidSettings {
    type Target = BoidSettings3D;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SkyBoidSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct BirdsPlugin;

impl Plugin for BirdsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkyBoidSettings>()
           .add_systems(Startup, load_bird_model)
           .add_systems(OnEnter(SimulationState::Sky), spawn_sky_birds)
           .add_systems(Update, (
                apply_sky_flocking,
                apply_forces_system,
                update_birds_position,
                confine_movement,
                adjust_population_birds,
                resize_boids,
                avoid_obstacles,
                confine_birds_movement,
                handle_mouse_input
           ).run_if(in_state(SimulationState::Sky)));
    }
}

fn apply_sky_flocking(
    boid_query: Query<(Entity, &Transform, &Velocity, &Boid), With<SkySceneMarker>>,
    event_writer: EventWriter<ApplyForceEvent>,
    boid_settings: Res<SkyBoidSettings>,
    groups_targets: Res<GroupsTargets>,
    kd_tree: Res<NNTree3D>,
) {
    let cohesion_range = boid_settings.cohesion_range;
    let alignment_range = boid_settings.alignment_range;
    let separation_range = boid_settings.separation_range;

    let event_writer = std::sync::Mutex::new(event_writer);

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
            entity,
            force: total_force 
        });
    });
}

pub fn update_birds_position(
    mut boid_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Boid>>,
    boid_settings: Res<SkyBoidSettings>,
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

fn load_bird_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model = asset_server.load("models/bird/scene.gltf#Scene0");
    commands.insert_resource(BirdModel(model));
}

fn spawn_sky_birds(
    mut commands: Commands,
    bird_model: Res<BirdModel>,
    boid_settings: Res<SkyBoidSettings>,
) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..boid_settings.count {
        let group = rng.gen_range(0..2);
        let random_pos = Vec3::new(
            rng.gen_range(-50.0..50.0),
            rng.gen_range(20.0..100.0),
            rng.gen_range(-50.0..50.0)
        );

        commands.spawn((
            BoidBundle {
                boid: Boid { group },
                velocity: Velocity { velocity: Vec3::new(1.0, 0.0, 0.0) },
                acceleration: Acceleration { acceleration: Vec3::ZERO },
                pbr_bundle: PbrBundle {
                    transform: Transform::from_translation(random_pos)
                        .with_scale(Vec3::splat(boid_settings.size * 0.3)),
                    ..default()
                },
                tracked_by_kdtree: TrackedByKDTree3D,
            },
            SkySceneMarker,
        ))
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: bird_model.0.clone(),
                transform: Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
                ..default()
            });
        });
    }
}

pub fn confine_birds_movement (
    mut boid_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Boid>>
) {
    let margin = BOUNDS_SIZE * 0.2;
    let x_min = -BOUNDS_SIZE + margin;
    let y_min = margin;
    let z_min = -BOUNDS_SIZE + margin;
    let x_max = BOUNDS_SIZE - margin;
    let y_max = 75.0 - margin;
    let z_max = BOUNDS_SIZE - margin;
    for (transform, mut velocity, _) in boid_query.iter_mut() {
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
}

fn adjust_population_birds(
    mut commands: Commands,
    mut boid_settings: ResMut<SkyBoidSettings>,
    query: Query<Entity, (With<Boid>, With<SkySceneMarker>)>,
    bird_model: Res<BirdModel>,
) {
    let current = boid_settings.count;
    let previous = boid_settings.previous_count;

    if current > previous {
        for _ in 0..(current - previous) {
            let mut rng = rand::thread_rng();
            let pos = Vec3::new(
                rng.gen_range(-BOUNDS_SIZE..BOUNDS_SIZE),
                rng.gen_range(-50.0..50.0),
                rng.gen_range(-BOUNDS_SIZE..BOUNDS_SIZE)
            );

            commands.spawn((
                BoidBundle {
                    boid: Boid { group: rng.gen_range(0..2) },
                    velocity: Velocity { velocity: Vec3::new(1.0, 0.0, 0.0) },
                    acceleration: Acceleration { acceleration: Vec3::ZERO },
                    pbr_bundle: PbrBundle {
                        transform: Transform::from_translation(pos)
                            .with_scale(Vec3::splat(boid_settings.size * 0.3)),
                        ..default()
                    },
                    tracked_by_kdtree: TrackedByKDTree3D,
                },
                SkySceneMarker,
            ))
            .with_children(|parent| {
                parent.spawn(SceneBundle {
                    scene: bird_model.0.clone(),
                    transform: Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
                    ..default()
                });
            });
        }
    } else if current < previous {
        for entity in query.iter().take(previous - current) {
            commands.entity(entity).despawn_recursive();
        }
    }
    
    boid_settings.previous_count = current;
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

fn handle_mouse_input(
    mouse_buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = get_mouse_world_position(&windows, &camera_query) {
            let obstacle_material = materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.2, 0.2),
                metallic: 0.0,
                perceptual_roughness: 0.9,
                ..default()
            });

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 50.0 })),
                    material: obstacle_material,
                    transform: Transform::from_translation(position),
                    ..default()
                },
                ObstacleTag,
                SkySceneMarker
            ));
        }
    }
}

fn get_mouse_world_position(
    windows: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) -> Option<Vec3> {
    let window = windows.get_single().ok()?;
    let (camera, camera_transform) = camera_query.get_single().ok()?;

    let cursor_position = window.cursor_position()?;
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (cursor_position / window_size) * 2.0 - Vec2::ONE;

    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));

    Some(world_position)
}