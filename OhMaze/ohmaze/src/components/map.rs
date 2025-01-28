use crate::components::tile::*;

pub struct Map {
    map: Vec<Vec<Vec<TileType>>>,
    pub current_level: usize,
}

impl Map {
    /// Creates a map from a config file where each pos represents a tile type and position,
    /// For each level, an empty line shall be used as a separator, including last one so that
    /// an additional push at the end is not needed
    ///
    /// # Examples
    ///
    /// WWWWW
    /// WSDFW
    /// WPEFW
    /// WIFFW
    /// WWWWW
    ///
    /// WWWWW
    /// WPEFW
    /// WIFFW
    /// WWWWW
    ///
    pub fn from(map_file: &str) -> Self {
        let contents = std::fs::read_to_string(map_file).expect("Could not open config file");
        let mut map = Vec::new();
        let mut level = Vec::new();

        for line in contents.lines() {
            if line.is_empty() {
                map.push(level);
                level = Vec::new();
                continue;
            }

            let mut row = Vec::new();
            for ch in line.chars() {
                let tile = match ch {
                    'F' => TileType::Empty,
                    'D' => TileType::Door,
                    'S' => TileType::Stairs,
                    'W' => TileType::Wall,
                    'I' => TileType::Item,
                    'P' => TileType::Player,
                    'E' => TileType::Enemy,
                    _ => unreachable!(),
                };
                row.push(tile);
            }

            level.push(row);
        }

        Map {
            map,
            current_level: 0,
        }
    }

    pub fn valid_pos(&self, pos_x: usize, pos_y: usize, key: bool) -> bool {
        match self.map[self.current_level][pos_x][pos_y] {
            TileType::Wall => false,
            TileType::Item => true, // Handle
            TileType::Door => key,  // Only valid if key
            _ => unreachable!(),
        }
    }
}
