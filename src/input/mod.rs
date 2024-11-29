use bevy::prelude::*;

pub mod systems;
pub use systems::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_buttons_input);
    }
}