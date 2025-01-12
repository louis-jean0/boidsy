use bevy::prelude::*;
use crate::ui::resources::SimulationState;

mod components;
mod systems;
mod marching_cubes;

pub use systems::*;

pub struct TerrainPlugin;

#[derive(Component)]
pub struct TerrainChunk {
    pub position: IVec3,
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Underwater), generate_terrain_chunks);
    }
}