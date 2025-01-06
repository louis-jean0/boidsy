use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::ui::resources::SimulationState;

pub mod components;
pub mod systems;
pub mod resources;
pub mod bundles;
pub mod events;

use events::ApplyForceEvent;
use resources::*;
use systems::*;

pub struct Boids3DPlugin;

impl Plugin for Boids3DPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraControlState>()
        .insert_resource(BoidSettings3D::default())
        .insert_resource(GroupsTargets::default())
        .insert_resource(MouseSettings::default())
        .add_event::<ApplyForceEvent>()
        .add_systems(Update, (
            flocking,
            apply_forces_system,
            update_boid_position,
            confine_movement,
            adjust_population,
            handle_camera_movement,
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
    NotShadowCaster,
    NotShadowReceiver
    ));

    let ground_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(BOUNDS_SIZE * 4.0))),
        material: ground_material,
        transform: Transform::from_xyz(0.0, -BOUNDS_SIZE, 0.0),
        ..default()
    });
}

#[derive(Resource)]
pub struct MouseSettings {
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.001,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

pub fn handle_camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut settings: ResMut<MouseSettings>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    camera_control_state: ResMut<CameraControlState>
) {
    if !camera_control_state.is_active {return;}
    let Ok(mut transform) = query.get_single_mut() else { return };
    
    for ev in mouse_motion.read() {
        settings.pitch = (settings.pitch - ev.delta.y * settings.sensitivity)
            .clamp(-1.54, 1.54); // Roughly PI/2
        settings.yaw -= ev.delta.x * settings.sensitivity;
    }

    let rotation = Quat::from_euler(EulerRot::YXZ, settings.yaw, settings.pitch, 0.0);
    transform.rotation = rotation;

    let mut movement = Vec3::ZERO;
    let mut speed = 500.0;

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        speed *= 2.0;
    }

    let forward = transform.forward();
    let right = transform.right();
    
    if keyboard_input.pressed(KeyCode::Z) {
        movement += forward;
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement -= forward;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        movement -= right;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement += right;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        movement.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        movement.y -= 1.0;
    }

    transform.translation += movement.normalize_or_zero() * speed * time.delta_seconds();
}