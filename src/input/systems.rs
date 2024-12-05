use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::input::resources::*;

use crate::boids_2d::systems::spawn_obstacle;


pub fn mouse_buttons_input(
    mouse_buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    shape_settings: Res<ShapeSettings>,
) {
    if mouse_buttons.pressed(MouseButton::Right) {
        println!("Clic droit détecté");
        if let Some(position) = cursor_position(&q_windows) {
            println!("Le curseur est dans la fenêtre principale, à {:?}", position);
            let window = q_windows.get_single().unwrap();
            spawn_obstacle(&mut commands, Vec2::new(position.x,window.height() - position.y), Vec3::new(1., 1., 1.), shape_settings.radius, &mut meshes, &mut materials);
        } else {
            println!("Le curseur n'est pas dans la fenêtre du jeu.");
        }
    }
}

pub fn cursor_position(
    q_windows: &Query<&Window, With<PrimaryWindow>>,
) -> Option<Vec2> {
    q_windows.single().cursor_position()
}