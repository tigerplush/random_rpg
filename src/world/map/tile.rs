use serde::{Deserialize, Serialize};

pub struct Tile {
    x: usize,
    y: usize,
    height: f64,
}

impl Tile {
    pub const fn new(x: usize, y: usize, height: f64) -> Self {
        Tile { x, y, height }
    }
}

impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string = format!("{},{},{}", self.x, self.y, self.height);
        serializer.serialize_str(string.as_str())
    }
}

impl<'de> Deserialize<'de> for Tile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tile_string = String::deserialize(deserializer)?;
        let values: Vec<String> = tile_string.split(',').map(|item| item.to_owned()).collect();
        let x = values[0].parse::<usize>().unwrap();
        let y = values[1].parse::<usize>().unwrap();
        let height = values[2].parse::<f64>().unwrap();
        Ok(Self { x, y, height })
    }
}
