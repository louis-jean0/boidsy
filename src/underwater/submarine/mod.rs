use bevy::prelude::*;
use crate::ui::resources::SimulationState;

pub mod components;
pub mod systems;

pub use systems::*;

pub struct SubmarinePlugin;

impl Plugin for SubmarinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Underwater), setup_submarine)
           .add_systems(Update, (
               submarine_movement,
               update_camera,
           ).run_if(in_state(SimulationState::Underwater)));
    }
}