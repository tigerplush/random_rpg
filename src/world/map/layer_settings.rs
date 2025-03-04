use noise::{BasicMulti, Simplex};
use serde::{Deserialize, Serialize};

/// Describes one layer of the map generation
///
/// To make more interesting maps it is recommended to have different
/// layers that serve different purposes, e.g. one layer for basic terrain,
/// another layer for mountain ranges, another layer for rivers etc.
#[derive(Serialize, Deserialize)]
pub struct LayerSettings {
    /// If you leave the seed `None` (or `null` in the settings file),
    /// a random seed will be generated for each of the layers from
    /// the main world seed
    pub seed: Option<u32>,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub frequency: f64,
}

impl LayerSettings {
    pub const fn set_frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self
    }

    pub const fn set_lacunarity(mut self, lacunarity: f64) -> Self {
        self.lacunarity = lacunarity;
        self
    }

    pub const fn set_octaves(mut self, octaves: usize) -> Self {
        self.octaves = octaves;
        self
    }
}

impl Default for LayerSettings {
    fn default() -> Self {
        LayerSettings {
            seed: None,
            octaves: BasicMulti::<Simplex>::DEFAULT_OCTAVES,
            lacunarity: BasicMulti::<Simplex>::DEFAULT_LACUNARITY,
            persistence: BasicMulti::<Simplex>::DEFAULT_PERSISTENCE,
            frequency: BasicMulti::<Simplex>::DEFAULT_FREQUENCY,
        }
    }
}
