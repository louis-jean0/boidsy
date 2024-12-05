use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::input::resources::*;

use crate::boids_2d::systems::*;
use crate::boids_2d::bundles::*;
use crate::boids_2d::components::*;
use bevy::input::mouse::MouseWheel;

use bevy::sprite::MaterialMesh2dBundle;


pub fn mouse_buttons_input(
    mouse_buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    shape_settings: Res<ShapeSettings>,
    keys: Res<Input<KeyCode>>,
    query: Query<Entity, (With<Position>, With<ColorMaterial>)>, // Ajout pour accéder aux obstacles
) {
    if mouse_buttons.pressed(MouseButton::Right) {
        if let Some(position) = cursor_position(&q_windows) {
            let window = q_windows.get_single().unwrap();
            spawn_obstacle(
                &mut commands,
                Vec2::new(position.x, window.height() - position.y),
                Vec3::new(1., 1., 1.),
                shape_settings.radius,
                &mut meshes,
                &mut materials,
            );
        }
    }

    if keys.just_pressed(KeyCode::R) {
        println!("R pressed");
        delete_obstacles(&mut commands, &query); // Appel de delete_obstacles ici
    }
}



pub fn cursor_position(
    q_windows: &Query<&Window, With<PrimaryWindow>>,
) -> Option<Vec2> {
    q_windows.single().cursor_position()
}

pub fn scroll_events(
    mut evr_scroll: EventReader<MouseWheel>,
    mut shape_settings: ResMut<ShapeSettings>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
            	if(ev.y>0.){
	            	if(shape_settings.radius < 100.0){
	            		shape_settings.radius+=1.;
	            	}
	            }
            	if(ev.y<0.){
            		if(shape_settings.radius > 0.0){
            		shape_settings.radius-=1.;
            		}
            	}    
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
        }
    }
}