use bevy::prelude::*;
use crate::underwater::{UnderwaterState};

pub mod components;
pub mod systems;


pub use systems::*;


pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UnderwaterState::Enabled), setup_environment)
           .add_systems(Update, (
               update_water_effects,
               spawn_particles,
           ).run_if(in_state(UnderwaterState::Enabled)));
    }
}