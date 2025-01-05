use bevy::prelude::*;
use crate::ui::resources::SimulationState;

pub mod components;
pub mod systems;
pub mod resources;
pub mod bundles;
pub mod events;

use events::ApplyForceEvent;
use resources::*;
use systems::*;

pub const BOIDS_COUNT: usize = 500; // Reduced initial count for 3D
pub const BOIDS_COHESION_RANGE: f32 = 30.0;
pub const BOIDS_ALIGNMENT_RANGE: f32 = 20.0;
pub const BOIDS_SEPARATION_RANGE: f32 = 15.0;

pub struct Boids3DPlugin;

impl Plugin for Boids3DPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoidSettings3D::new(
            BOIDS_COUNT,
            BOIDS_ALIGNMENT_RANGE,
            BOIDS_COHESION_RANGE,
            BOIDS_SEPARATION_RANGE
        ))
        .insert_resource(GroupsTargets::default())
        .add_event::<ApplyForceEvent>()
        .add_systems(Update, (
            flocking,
            apply_forces_system,
            update_boid_position,
            confine_movement,
            adjust_population,
            handle_camera_movement, // New system for 3D camera control
        ).run_if(in_state(SimulationState::Mode3D)));
    }
}

pub fn setup_3d_scene(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    let boundary_material = materials.add(StandardMaterial {
        base_color: Color::rgba(0.1, 0.1, 0.1, 0.05),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(
            BOUNDS_SIZE * 2.0,
            BOUNDS_SIZE * 2.0,
            BOUNDS_SIZE * 2.0,
        ))),
        material: boundary_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

pub fn handle_camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let Ok(mut camera_transform) = query.get_single_mut() else { return };
    
    let mut movement = Vec3::ZERO;
    let speed = 50.0;

    if keyboard_input.pressed(KeyCode::Z) {
        movement.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        movement.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::E) {
        movement.y += 1.0;
    }

    camera_transform.translation += movement.normalize_or_zero() * speed * time.delta_seconds();
}