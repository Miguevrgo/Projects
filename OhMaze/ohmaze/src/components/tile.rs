use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Door,
    Item,
    Player,
    Enemy,
    UpStairs,
    DownStairs,
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            TileType::Empty => 'F',
            TileType::Door => 'D',
            TileType::UpStairs => 'S',
            TileType::DownStairs => 's',
            TileType::Wall => 'W',
            TileType::Item => 'I',
            TileType::Player => 'P',
            TileType::Enemy => 'E',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub is_visible: bool,
    pub is_blocking: bool,
}
