use crate::entities::item::Item;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub pos_x: usize,
    pub pos_y: usize,
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

    pub fn has_key(&self, key_id: &usize) -> bool {
        self.inventory
            .iter()
            .any(|item| matches!(item, Item::Key(id) if id == key_id))
    }

    pub fn move_by(&mut self, x_axis: usize, y_axis: usize) {
        self.pos_x += x_axis;
        self.pos_y += y_axis;
    }
}
