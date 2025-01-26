use crate::components::player::Player;
use bevy::prelude::*;

pub fn handle_movement(input: Res<ButtonInput<KeyCode>>, mut player: &mut Player) {
    let (mut dx, mut dy) = (0, 0);

    if input.just_pressed(KeyCode::KeyW) {
        dy += 1;
    } else if input.just_pressed(KeyCode::KeyS) {
        dy -= 1;
    } else if input.just_pressed(KeyCode::KeyA) {
        dx -= 1;
    } else if input.just_pressed(KeyCode::KeyD) {
        dx += 1;
    }

    player.move_by(dx, dy);
}
