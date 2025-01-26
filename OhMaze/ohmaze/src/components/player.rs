use crate::components::item::Item;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub pos_x: i16,
    pub pos_y: i16,
    pub inventory: Vec<Item>,
}

impl Player {
    pub fn spawn() -> Self {
        Player {
            pos_x: 0,
            pos_y: 0,
            inventory: Vec::new(),
        }
    }

    pub fn add_to_inventory(&mut self, item: Item) {
        self.inventory.push(item);
    }

    pub fn has_key(&self, key_id: &u8) -> bool {
        self.inventory
            .iter()
            .any(|item| matches!(item, Item::Key(id) if id == key_id))
    }

    /// TODO: Implement is_valid_position
    pub fn move_by(&mut self, x_axis: i16, y_axis: i16) {
        ///TODO: Delete this function, and implement it correctly elsewhere
        let is_valid_position =
            |x: i16, y: i16| -> bool { (0..10).contains(&x) && (0..10).contains(&y) };
        if is_valid_position(self.pos_x + x_axis, self.pos_y + y_axis) {
            self.pos_x += x_axis;
            self.pos_y += y_axis;
        }
    }
}
