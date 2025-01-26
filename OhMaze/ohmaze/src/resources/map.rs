use std::path::Path;

struct Map {
    // TODO: Tiles not u32
    map: Vec<Vec<Vec<u32>>>,
    current_level: u8,
    //TODO: Maybe store player here so that it restored to its position.
}

impl Map {
    pub fn from(config_file: &Path) -> Self {
        //TODO: How the fvck do we serialize and deserialize the map? Serde could help but chars
        //for each pos? maybe a number? We just need like 3 or 4 bits
        Map {
            map: Vec::new(),
            current_level: 0,
        }
    }
}
