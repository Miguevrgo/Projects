use crate::components::player::Player;
use bevy::prelude::*;

pub fn handle_movement(input: Res<ButtonInput<KeyCode>>, mut player: &mut Player) {
    let (mut dx, mut dy) = (0, 0);

    if input.just_pressed(KeyCode::W) {
        dy += 1;
    } else if input.just_pressed(KeyCode::S) {
        dy -= 1;
    } else if input.just_pressed(KeyCode::A) {
        dx -= 1;
    } else if input.just_pressed(KeyCode::D) {
        dx += 1;
    }
}
