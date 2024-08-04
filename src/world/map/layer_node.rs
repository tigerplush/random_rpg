use noise::{Clamp, Curve, Fbm, Min, MultiFractal, NoiseFn, Perlin, ScaleBias};
use serde::{Deserialize, Serialize};

use crate::map::min_settings;

use super::{
    clamp_settings::ClampSettings,
    layer_settings::LayerSettings,
    min_settings::MinSettings,
    scale_settings::{self, ScaleSettings},
    terrain_shape::TerrainShape,
};

#[derive(Serialize, Deserialize)]
pub struct LayerNode {
    pub id: String,
    pub layer_type: LayerType,
}

#[derive(Serialize, Deserialize)]
pub enum LayerType {
    SimpleLayer(LayerSettings),
    TerrainShape(TerrainShape),
    ScaleBias(ScaleSettings),
    Min(MinSettings),
    Clamp(ClampSettings),
}

impl LayerNode {
    pub fn new(id: &str, layer_type: LayerType) -> Self {
        LayerNode {
            id: String::from(id),
            layer_type,
        }
    }

    pub fn resolve(&self, pool: &Vec<LayerNode>) -> Box<dyn NoiseFn<f64, 3>> {
        match &self.layer_type {
            LayerType::Clamp(clamp_settings) => {
                let Some(next_node) = pool.iter().find(|node| node.id == clamp_settings.input)
                else {
                    panic!(
                        "Node {} could not find input node {}",
                        self.id, clamp_settings.input
                    );
                };
                let source = next_node.resolve(pool);
                Self::clamp(source, clamp_settings)
            }
            LayerType::ScaleBias(scale_settings) => {
                let Some(next_node) = pool
                    .iter()
                    .find(|node: &&LayerNode| node.id == scale_settings.input)
                else {
                    panic!(
                        "Node {} could not find input node {}",
                        self.id, scale_settings.input
                    );
                };
                let source = next_node.resolve(pool);
                Self::scale(source, scale_settings)
            }
            LayerType::SimpleLayer(layer_settings) => Self::resolve_simple_layer(layer_settings),
            LayerType::Min(min_settings) => {
                let Some(lhs) = pool
                    .iter()
                    .find(|node: &&LayerNode| node.id == min_settings.lhs)
                else {
                    panic!(
                        "Node {} could not find input node {}",
                        self.id, min_settings.lhs
                    );
                };
                let Some(rhs) = pool
                    .iter()
                    .find(|node: &&LayerNode| node.id == min_settings.rhs)
                else {
                    panic!(
                        "Node {} could not find input node {}",
                        self.id, min_settings.rhs
                    );
                };
                Self::min(lhs.resolve(pool), rhs.resolve(pool))
            }
            LayerType::TerrainShape(shape_settings) => {
                let Some(next_node) = pool
                    .iter()
                    .find(|node: &&LayerNode| node.id == shape_settings.input)
                else {
                    panic!(
                        "Node {} could not find input node {}",
                        self.id, shape_settings.input
                    );
                };
                Self::curve(next_node.resolve(pool), shape_settings)
            }
            _ => unimplemented!(),
        }
    }

    fn resolve_simple_layer(layer_settings: &LayerSettings) -> Box<dyn NoiseFn<f64, 3>> {
        Box::new(
            Fbm::<Perlin>::new(0)
                .set_frequency(layer_settings.frequency)
                .set_persistence(layer_settings.persistence)
                .set_lacunarity(layer_settings.lacunarity)
                .set_octaves(layer_settings.octaves),
        )
    }

    fn clamp(
        source: Box<dyn NoiseFn<f64, 3>>,
        clamp_settings: &ClampSettings,
    ) -> Box<dyn NoiseFn<f64, 3>> {
        Box::new(
            Clamp::new(source).set_bounds(clamp_settings.lower_bounds, clamp_settings.upper_bounds),
        )
    }

    fn scale(
        source: Box<dyn NoiseFn<f64, 3>>,
        scale_settings: &ScaleSettings,
    ) -> Box<dyn NoiseFn<f64, 3>> {
        Box::new(
            ScaleBias::new(source)
                .set_bias(scale_settings.bias)
                .set_scale(scale_settings.scale),
        )
    }

    fn min(
        lhs: Box<dyn NoiseFn<f64, 3>>,
        rhs: Box<dyn NoiseFn<f64, 3>>,
    ) -> Box<dyn NoiseFn<f64, 3>> {
        Box::new(Min::new(lhs, rhs))
    }

    fn curve(
        source: Box<dyn NoiseFn<f64, 3>>,
        shape_settings: &TerrainShape,
    ) -> Box<dyn NoiseFn<f64, 3>> {
        let mut curve = Curve::new(source);
        for point in &shape_settings.control_points {
            curve = curve.add_control_point(point.x, point.y);
        }
        Box::new(curve)
    }

    pub fn continent_bundle() -> Vec<LayerNode> {
        vec![
            LayerNode::new("fb0", LayerType::SimpleLayer(LayerSettings::default())),
            LayerNode::new(
                "cu",
                LayerType::TerrainShape(TerrainShape::default().with_input("fb0")),
            ),
            LayerNode::new("fb1", LayerType::SimpleLayer(LayerSettings::default())),
            LayerNode::new(
                "sb",
                LayerType::ScaleBias(ScaleSettings::new("fb1", 0.375, 0.625)),
            ),
            LayerNode::new("mi", LayerType::Min(MinSettings::new("sb", "cu"))),
            LayerNode::new("cl", LayerType::Clamp(ClampSettings::new("mi", -1.0, 1.0))),
        ]
    }
}
