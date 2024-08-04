use serde::{Deserialize, Serialize};

use crate::utilities::UVec2;

use super::{color_gradient::GradientPoint, layer_settings::LayerSettings};

/// Describes the world map
#[derive(Serialize, Deserialize)]
pub struct MapSettings {
    /// Size of the world map in pixels. Default is 1024x512
    /// Which is roughly 1/40th of the size of earth.
    /// Large values will lead to longer generation times.
    /// Any dimension has to be less than 32767
    pub size: UVec2,
    pub layers: Vec<LayerSettings>,
    pub color_gradient: Vec<GradientPoint>,
}

impl Default for MapSettings {
    fn default() -> Self {
        MapSettings {
            // earth is roughly 40075km x 20014km, but the noise algorithm takes a long time
            // for large values, so we return 1024x512 (aspect ratio 2:1)
            size: UVec2::new(1024, 512),
            layers: vec![LayerSettings::default()],
            // these are the default color gradient points from noise.rs
            color_gradient: vec![
                GradientPoint::new(-1.0, 0, 0, 0),
                GradientPoint::new(-256.0 / 16384.0, 6, 58, 127),
                GradientPoint::new(-1.0 / 16384.0, 14, 112, 192),
                GradientPoint::new(0.0, 70, 120, 60),
                GradientPoint::new(1024.0 / 16384.0, 110, 140, 75),
                GradientPoint::new(2048.0 / 16384.0, 160, 140, 111),
                GradientPoint::new(3072.0 / 16384.0, 184, 163, 141),
                GradientPoint::new(4096.0 / 16384.0, 128, 128, 128),
                GradientPoint::new(5632.0 / 16384.0, 128, 128, 128),
                GradientPoint::new(6144.0 / 16384.0, 250, 250, 250),
                GradientPoint::new(1.0, 255, 255, 255),
            ],
        }
    }
}
