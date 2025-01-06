use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Mode2D,
    Mode3D,
    Underwater
}