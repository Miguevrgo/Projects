use bevy::prelude::Resource;

use crate::components::tile::*;

#[derive(Resource)]
pub struct Map {
    map: Vec<Vec<Vec<TileType>>>,
    pub player_pos: (usize, usize),
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
                    'S' => TileType::UpStairs,
                    's' => TileType::DownStairs,
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

        let mut player_pos = (0, 0);

        for i in 0..level.len() {
            for j in 0..level[i].len() {
                if level[i][j] == TileType::Player {
                    player_pos = (i, j);
                }
            }
        }

        Map {
            map,
            player_pos,
            current_level: 0,
        }
    }

    pub fn valid_pos(&mut self, pos_x: usize, pos_y: usize, key: bool) -> bool {
        match self.map[self.current_level][pos_x][pos_y] {
            TileType::Wall => false,
            TileType::Item => true, // Handle
            TileType::Door => key,  // Only valid if key
            TileType::UpStairs => {
                self.switch_level(true);
                true
            }
            TileType::DownStairs => {
                self.switch_level(false);
                false
            }
            _ => unreachable!(),
        }
    }

    /// TODO: In order to allow a stair to be in a different position in each level some kind of
    /// movement of player shall be allowed or maybe the player is just rendered where the P in the
    /// map is which I think is not very efficient
    fn switch_level(&mut self, going_up: bool) {
        self.current_level += going_up as usize;
    }

    pub fn move_player(&mut self, dx: isize, dy: isize, key: bool) {
        let (x, y) = self.player_pos;

        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0
            && new_y >= 0
            && (new_x as usize) < self.map[self.current_level].len()
            && (new_y as usize) < self.map[self.current_level][new_x as usize].len()
            && self.valid_pos(new_x as usize, new_y as usize, key)
        {
            self.map[self.current_level][x][y] = TileType::Empty;
            self.map[self.current_level][new_x as usize][new_y as usize] = TileType::Player;
            self.player_pos = (new_x as usize, new_y as usize);
        }
    }

    pub fn render_map(&self) {
        for tile in self.map[self.current_level].iter() {
            for t in tile.iter() {
                print!("{t}");
            }
            println!();
        }
    }
}
