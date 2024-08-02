use serde::{Deserialize, Serialize};

use crate::utilities::UVec2;

#[derive(Serialize, Deserialize)]
pub struct MapSettings {
    size: UVec2,
}

impl MapSettings {
    pub const fn get_world_width(&self) -> usize {
        self.size.x
    }

    pub const fn get_world_height(&self) -> usize {
        self.size.y
    }
}

impl Default for MapSettings {
    fn default() -> Self {
        MapSettings {
            // earth is roughly 40075km x 20014km, but the noise algorithm takes a long time
            // for large values, so we return 400x200
            size: UVec2::new(400, 200),
        }
    }
}
