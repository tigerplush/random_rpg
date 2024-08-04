use std::path::PathBuf;

use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    NoiseFn,
};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::utilities::UVec2;

use super::{
    color_gradient::GradientPoint,
    layer_node::{LayerNode, LayerType},
};

/// Describes the world map
#[derive(Serialize, Deserialize)]
pub struct MapSettings {
    /// Size of the world map in pixels. Default is 1024x512
    /// Which is roughly 1/40th of the size of earth.
    /// Large values will lead to longer generation times.
    /// Any dimension has to be less than 32767
    pub size: UVec2,
    pub layers: Vec<LayerNode>,
    pub color_gradient: Vec<GradientPoint>,
}

impl MapSettings {
    pub fn generate(&self, seed: u32, path: &PathBuf) -> Box<dyn NoiseFn<f64, 3>> {
        let Some(last_node) = self.layers.last() else {
            panic!("You need to specify at least one layer");
        };

        let mut rng = StdRng::seed_from_u64(seed as u64);

        self.resolve(last_node, &mut rng, path)
    }

    fn resolve(
        &self,
        current_node: &LayerNode,
        rng: &mut StdRng,
        path: &PathBuf,
    ) -> Box<dyn NoiseFn<f64, 3>> {
        let result = match &current_node.layer_type {
            LayerType::Clamp(clamp_settings) => {
                let Some(next_node) = self
                    .layers
                    .iter()
                    .find(|node| node.id == clamp_settings.input)
                else {
                    panic!(
                        "Node {} could not find input node {}",
                        current_node.id, clamp_settings.input
                    );
                };
                let source = self.resolve(next_node, rng, path);
                LayerNode::clamp(source, clamp_settings)
            },
            LayerType::ScaleBias(scale_settings) => {
                let Some(next_node) = self.layers
                    .iter()
                    .find(|node: &&LayerNode| node.id == scale_settings.input)
                else {
                    panic!(
                        "Node {} could not find input node {}",
                        current_node.id, scale_settings.input
                    );
                };
                let source = self.resolve(next_node, rng, path);
                LayerNode::scale(source, scale_settings)
            },
            LayerType::SimpleLayer(layer_settings) => {
                // make sure a random number always is consumed so if
                // one layer has its own seed the next layer still gets the same
                // random seed
                let random_seed = rng.next_u32();
                let seed = match layer_settings.seed {
                    Some(seed) => seed,
                    None => random_seed,
                };
                LayerNode::simple_layer(layer_settings, seed)
            },
            LayerType::Min(min_settings) => {
                let Some(lhs) = self.layers
                    .iter()
                    .find(|node: &&LayerNode| node.id == min_settings.lhs)
                else {
                    panic!(
                        "Node {} could not find input node {}",
                        current_node.id, min_settings.lhs
                    );
                };
                let Some(rhs) = self.layers
                    .iter()
                    .find(|node: &&LayerNode| node.id == min_settings.rhs)
                else {
                    panic!(
                        "Node {} could not find input node {}",
                        current_node.id, min_settings.rhs
                    );
                };
                let lhs_result = self.resolve(lhs, rng, path);
                let rhs_result = self.resolve(rhs, rng, path);
                LayerNode::min(lhs_result, rhs_result)
            },
            LayerType::TerrainShape(shape_settings) => {
                let Some(next_node) = self.layers
                    .iter()
                    .find(|node: &&LayerNode| node.id == shape_settings.input)
                else {
                    panic!(
                        "Node {} could not find input node {}",
                        current_node.id, shape_settings.input
                    );
                };
                let result = self.resolve(next_node, rng, path);
                LayerNode::curve(result, shape_settings)
            },
        };

        PlaneMapBuilder::new(&result)
            .set_size(self.size.x, self.size.y)
            .build()
            .write_to_file(&path.join(format!("layer_{}.png", current_node.id)));

        result
    }
}

impl Default for MapSettings {
    fn default() -> Self {
        MapSettings {
            // earth is roughly 40075km x 20014km, but the noise algorithm takes a long time
            // for large values, so we return 1024x512 (aspect ratio 2:1)
            size: UVec2::new(1024, 512),
            layers: LayerNode::continent_bundle(),
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
