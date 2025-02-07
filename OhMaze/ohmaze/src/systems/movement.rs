use crate::components::map::Map;
use crate::entities::player::Player;
use bevy::prelude::*;

pub fn handle_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut map: ResMut<Map>,
    player: Res<Player>,
) {
    let mut dx: isize = 0;
    let mut dy: isize = 0;

    if input.pressed(KeyCode::KeyW) {
        dx = -1;
    } else if input.pressed(KeyCode::KeyS) {
        dx = 1;
    } else if input.pressed(KeyCode::KeyA) {
        dy = -1;
    } else if input.pressed(KeyCode::KeyD) {
        dy = 1;
    }
    let has_key = player.has_key(&map.current_level);
    if dx != 0 || dy != 0 {
        map.move_player(dx, dy, has_key);
    }
}
