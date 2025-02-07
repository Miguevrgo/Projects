use crate::entities::item::Item;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Player {
    pub inventory: Vec<Item>,
}

impl Player {
    pub fn spawn() -> Self {
        Player {
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
}
