use bevy::prelude::*;

pub mod systems;
pub mod resources;
pub use systems::*;
pub use resources::*;

pub const RADIUS: f32 = 10.0;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShapeSettings::new(RADIUS))
        .add_systems(Update, mouse_buttons_input);
    }
}