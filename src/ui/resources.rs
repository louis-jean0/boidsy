use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Mode2D,
    #[default]
    Mode3D,
    Underwater,
    Sky
}