use serde::{Deserialize, Serialize};

use crate::utilities::UVec2;

use super::layer_settings::LayerSettings;

#[derive(Serialize, Deserialize)]
pub struct MapSettings {
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
