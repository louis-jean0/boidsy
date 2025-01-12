use bevy::prelude::*;
use components::UnderwaterEffect;

pub mod components;
pub mod systems;


pub use systems::*;

use crate::ui::resources::SimulationState;


pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnderwaterEffect>()
           .add_systems(OnEnter(SimulationState::Underwater), setup_environment)
           .add_systems(Update, (
               spawn_particles,
               update_bubbles
           ).run_if(in_state(SimulationState::Underwater)));
    }
}