use serde::{Deserialize, Serialize};

use crate::utilities::Vec2;

/// Shapes the terrain from a list of control points
///
/// Because terrain shaping is done by cubic splines, every
/// TerrainShape needs to have four or more control points.
///
/// Noise will generate output between -1.0 and 1.0. To create a mountain
/// with height 100.0, you would need to add a control point (1.0, 100.0)
#[derive(Serialize, Deserialize)]
pub struct TerrainShape {
    pub input: String,
    pub control_points: Vec<Vec2>,
}

impl TerrainShape {
    pub fn with_input(mut self, input: &str) -> Self {
        self.input = String::from(input);
        self
    }
}

impl Default for TerrainShape {
    fn default() -> Self {
        TerrainShape {
            input: String::new(),
            control_points: vec![
                Vec2::new(-2.0, -1.625),
                Vec2::new(-1.0, -1.375),
                Vec2::new(0.0, -0.375),
                Vec2::new(0.0625, 0.125),
                Vec2::new(0.125, 0.250),
                Vec2::new(0.25, 1.0),
                Vec2::new(0.5, 0.25),
                Vec2::new(0.75, 0.25),
                Vec2::new(1.0, 0.5),
                Vec2::new(2.0, 0.5),
            ],
        }
    }
}
