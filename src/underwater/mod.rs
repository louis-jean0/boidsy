use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

mod submarine;
mod terrain;
mod environment;

use submarine::SubmarinePlugin;
use terrain::TerrainPlugin;
use environment::EnvironmentPlugin;

use crate::{cleanup_3d_mode, ui::resources::SimulationState};

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum UnderwaterState {
    #[default]
    Disabled,
    Enabled,
}

pub struct UnderwaterPlugin;

impl Plugin for UnderwaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UnderwaterState>()
           .add_plugins((
               SubmarinePlugin,
               TerrainPlugin,
               EnvironmentPlugin,
           ))
           .add_systems(OnEnter(SimulationState::Underwater), (
            setup_underwater_scene,
            cleanup_3d_mode))
           .add_systems(OnExit(SimulationState::Underwater), cleanup_underwater_scene);
    }
}

#[derive(Component)]
pub struct UnderwaterMarker;

pub fn setup_underwater_scene(
    mut commands: Commands
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-100.0, 2.0, -100.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 1,
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.1, 0.1, 0.5)),
                ..default()
            },
            ..default()
        },
        // Add fog
        FogSettings {
            color: Color::rgb(0.25, 0.25, 0.75),
            falloff: FogFalloff::Linear { 
                start: 5.0, 
                end: 100.0 
            },
            ..default()
        },
        UnderwaterMarker
    ));

    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            directional_light: DirectionalLight {
                color: Color::WHITE,
                illuminance: 10000.0,
                ..default()
            },
            ..default()
        },
        UnderwaterMarker
    ));
}

fn cleanup_underwater_scene(
    mut commands: Commands,
    query: Query<Entity, With<UnderwaterMarker>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
