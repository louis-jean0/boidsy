use bevy::prelude::*;
use crate::underwater::{UnderwaterState};

pub mod components;
pub mod systems;


pub use systems::*;
pub struct SubmarinePlugin;

impl Plugin for SubmarinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            submarine_movement.run_if(in_state(UnderwaterState::Enabled))
        );
    }
}