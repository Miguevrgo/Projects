struct Map {
    // TODO: Tiles not u32
    map: Vec<Vec<Vec<u32>>>,
    current_level: usize,
    //TODO: Maybe store player here so that it restored to its position.
}

impl Map {
    pub fn from(config_file: &str) -> Self {
        //TODO: How the fvck do we serialize and deserialize the map? Serde could help but chars
        //for each pos? maybe a number? We just need like 3 or 4 bits
        let contents = std::fs::read_to_string("{config_file}").unwrap();

        Map {
            map: Vec::new(),
            current_level: 0,
        }
    }

    pub fn valid_pos(&self, pos_x: usize, pos_y: usize, key: bool) -> bool {
        //TODO: Use tiles instead of numbers
        const WALL: u32 = 2;
        const ITEM: u32 = 3;
        const DOOR: u32 = 4;
        match self.map[self.current_level][pos_x][pos_y] {
            WALL => false,
            ITEM => true, // Handle
            DOOR => key,  // Only valid if key
            _ => unreachable!(),
        }
    }
}
