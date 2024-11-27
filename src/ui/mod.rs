use bevy::prelude::*;

pub mod systems;
pub use systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_ui);
    }
}