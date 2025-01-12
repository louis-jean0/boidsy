use bevy::prelude::*;

mod submarine;
mod terrain;
mod environment;
mod fish;

use submarine::SubmarinePlugin;
use terrain::TerrainPlugin;
use environment::EnvironmentPlugin;
use fish::FishPlugin;

use crate::{cleanup_3d_mode, ui::resources::SimulationState};

pub struct UnderwaterPlugin;

impl Plugin for UnderwaterPlugin {
    fn build(&self, app: &mut App) {
        app
           .add_plugins((
               SubmarinePlugin,
               TerrainPlugin,
               EnvironmentPlugin,
               FishPlugin,
           ))
           .add_systems(OnEnter(SimulationState::Underwater), cleanup_3d_mode)
           .add_systems(OnExit(SimulationState::Underwater), cleanup_underwater_scene);
    }
}

#[derive(Component)]
pub struct UnderwaterMarker;

fn cleanup_underwater_scene(
    mut commands: Commands,
    query: Query<Entity, With<UnderwaterMarker>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
