use bevy::prelude::*;

pub fn mouse_buttons_input(mouse_buttons: Res<Input<MouseButton>>) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        
    }
}

pub fn right_mouse_button_pressed(mouse_buttons: Res<Input<MouseButton>>) -> bool {
    mouse_buttons.just_pressed(MouseButton::Right)
}