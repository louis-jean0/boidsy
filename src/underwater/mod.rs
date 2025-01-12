use bevy::prelude::*;

mod submarine;
mod terrain;
mod environment;

use submarine::SubmarinePlugin;
use terrain::TerrainPlugin;
use environment::EnvironmentPlugin;

use crate::{cleanup_3d_mode, ui::resources::SimulationState};

pub struct UnderwaterPlugin;

impl Plugin for UnderwaterPlugin {
    fn build(&self, app: &mut App) {
        app
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
