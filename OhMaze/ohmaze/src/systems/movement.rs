use crate::components::map::Map;
use crate::entities::player::*;
use bevy::prelude::*;

pub fn handle_movement(input: Res<ButtonInput<KeyCode>>, player: &mut Player, map: &Map) {
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

    let new_x = player.pos_x as isize + dx;
    let new_y = player.pos_y as isize + dy;

    if new_x >= 0 && new_y >= 0 {
        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if map.valid_pos(new_x, new_y, player.has_key(&map.current_level)) {
            player.move_by(dx as usize, dy as usize);
        }
    }
}
