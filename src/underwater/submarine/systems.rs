use bevy::prelude::*;

use crate::underwater::submarine::components::*;

pub fn submarine_movement(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut sub_query: Query<(&mut Transform, &Submarine)>,
) {
    let Ok((mut transform, submarine)) = sub_query.get_single_mut() else { return };
    let speed = submarine.speed * time.delta_seconds();

    let movement = if keyboard.pressed(KeyCode::W) {
        transform.forward()
    }
    else if keyboard.pressed(KeyCode::S) {
        transform.back()
    } else {
        Vec3::ZERO
    };

    transform.translation += movement * speed;
}