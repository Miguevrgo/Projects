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
}
