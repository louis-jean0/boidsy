use bevy::prelude::*;

#[derive(Event)]
pub struct CursorVisibilityEvent {
    pub visible: bool
}