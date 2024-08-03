use noise::{BasicMulti, Simplex};
use serde::{Deserialize, Serialize};

use super::terrain_shape::TerrainShape;

#[derive(Serialize, Deserialize)]
pub struct LayerSettings {
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub frequency: f64,
    pub terrain_shaping: TerrainShape,
}

impl Default for LayerSettings {
    fn default() -> Self {
        LayerSettings {
            octaves: BasicMulti::<Simplex>::DEFAULT_OCTAVES,
            lacunarity: BasicMulti::<Simplex>::DEFAULT_LACUNARITY,
            persistence: BasicMulti::<Simplex>::DEFAULT_PERSISTENCE,
            frequency: BasicMulti::<Simplex>::DEFAULT_FREQUENCY,
            terrain_shaping: TerrainShape::default(),
        }
    }
}
