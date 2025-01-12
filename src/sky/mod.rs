use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use crate::{input::handle_camera_movement, ui::resources::SimulationState};

pub mod environment;
pub use environment::*;

pub mod birds;
use birds::BirdsPlugin;

#[derive(Component)]
pub struct SkySceneMarker;

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EnvironmentPlugin,
            BirdsPlugin,
        ))
        .add_systems(Update, handle_camera_movement)
        .add_systems(OnEnter(SimulationState::Sky), setup_sky_scene)
        .add_systems(OnExit(SimulationState::Sky), cleanup_sky_scene);
    }
}

pub fn setup_sky_scene(
    mut commands: Commands
) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-50.0, 20.0, -50.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 2,
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.3, 0.1, 0.6)),
                ..default()
            },
            ..default()
        },
        SkySceneMarker
    ));

    // Lighting
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 15000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 10.0, 0.0)
                .looking_at(Vec3::new(-0.5, -1.0, -0.5), Vec3::Y),
            ..default()
        },
        SkySceneMarker
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        brightness: 0.3,
        ..default()
    });
}

fn cleanup_sky_scene(
    mut commands: Commands,
    query: Query<Entity, With<SkySceneMarker>>,
) {
    println!("Cleaning up sky scene");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
