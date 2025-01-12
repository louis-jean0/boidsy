use bevy::prelude::*;
use crate::{boids_2d::components::ObstacleTag, boids_3d::systems::BOUNDS_SIZE, ui::resources::SimulationState};
use super::SkySceneMarker;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Sky), setup_environment);
    }
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let platform_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.2, 0.5, 0.3),
        metallic: 0.0,
        perceptual_roughness: 0.9,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(BOUNDS_SIZE, 1.0, BOUNDS_SIZE))),
            material: platform_material.clone(),
            transform: Transform::from_xyz(0.0, -50.0, 0.0),
            ..default()
        },
        SkySceneMarker,
        ObstacleTag
    ));

    let island_materials = [
        materials.add(StandardMaterial {
            base_color: Color::rgb(0.48, 0.3, 0.2),
            metallic: 0.0,
            perceptual_roughness: 1.0,
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.4, 0.2),
            metallic: 0.0,
            perceptual_roughness: 0.8,
            ..default()
        }),
    ];

    // Increase the radius and adjust the angle increment to disperse the islands more
    let base_radius = 100.0;
    let angle_increment = std::f32::consts::PI * 0.125; // Adjust the angle increment for more dispersion
    let base_height = -50.0; // Base height to ensure islands are above the ground plane

    for i in 0..20 {
        let angle = i as f32 * angle_increment;
        let radius = base_radius + (i as f32 * 10.0).sin() * 40.0; // Increase the base radius and variation
        let height = base_height + 20.0 + (i as f32 * 1.5).sin() * 100.0; // Ensure height is above -50
        if height < -50.0 {
            continue;
        }
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 8.0 + (i as f32 * 0.8).sin() * 4.0,
                    height: 5.0,
                    resolution: 20,
                    segments: 20,
                })),
                material: island_materials[i % 2].clone(),
                transform: Transform::from_xyz(
                    angle.cos() * radius,
                    height,
                    angle.sin() * radius,
                )
                .with_rotation(Quat::from_rotation_z(angle * 0.2)),
                ..default()
            },
            SkySceneMarker,
            ObstacleTag
        ));
    }

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 20000.0,
                shadows_enabled: true,
                color: Color::rgb(1.0, 0.95, 0.8),
                ..default()
            },
            transform: Transform::from_xyz(50.0, 200.0, 50.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        SkySceneMarker,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.7, 0.8, 1.0),
        brightness: 0.3,
    });
}
