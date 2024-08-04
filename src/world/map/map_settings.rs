use serde::{Deserialize, Serialize};

use crate::utilities::UVec2;

use super::layer_settings::LayerSettings;

/// Describes the world map
#[derive(Serialize, Deserialize)]
pub struct MapSettings {
    /// Size of the world map in pixels. Default is 400x200
    /// Which is roughly 1/100th of the size of earth.
    /// Large values will lead to longer generation times.
    /// Any dimension has to be less than 32767
    pub size: UVec2,
    pub layers: Vec<LayerSettings>,
}

impl Default for MapSettings {
    fn default() -> Self {
        MapSettings {
            // earth is roughly 40075km x 20014km, but the noise algorithm takes a long time
            // for large values, so we return 400x200
            size: UVec2::new(400, 200),
            layers: vec![LayerSettings::default()],
        }
    }
}
