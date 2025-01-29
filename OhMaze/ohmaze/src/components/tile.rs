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

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub is_visible: bool,
    pub is_blocking: bool,
}
