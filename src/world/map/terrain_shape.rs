use serde::{Deserialize, Serialize};

use crate::utilities::Vec2;

#[derive(Serialize, Deserialize)]
pub struct TerrainShape {
    pub control_points: Vec<Vec2>,
}

impl Default for TerrainShape {
    fn default() -> Self {
        TerrainShape {
            control_points: vec![
                Vec2::new(-2.0, -2.0),
                Vec2::new(-1.0, -1.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(2.0, 2.0),
            ],
        }
    }
}
