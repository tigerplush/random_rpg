use noise::{Clamp, Curve, Fbm, Min, MultiFractal, NoiseFn, Perlin, ScaleBias};
use serde::{Deserialize, Serialize};

use super::{
    clamp_settings::ClampSettings, layer_settings::LayerSettings, min_settings::MinSettings,
    scale_settings::ScaleSettings, terrain_shape::TerrainShape,
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

    pub fn simple_layer(layer_settings: &LayerSettings, seed: u32) -> Box<dyn NoiseFn<f64, 3>> {
        Box::new(
            Fbm::<Perlin>::new(seed)
                .set_frequency(layer_settings.frequency)
                .set_persistence(layer_settings.persistence)
                .set_lacunarity(layer_settings.lacunarity)
                .set_octaves(layer_settings.octaves),
        )
    }

    pub fn clamp(
        source: Box<dyn NoiseFn<f64, 3>>,
        clamp_settings: &ClampSettings,
    ) -> Box<dyn NoiseFn<f64, 3>> {
        Box::new(
            Clamp::new(source).set_bounds(clamp_settings.lower_bounds, clamp_settings.upper_bounds),
        )
    }

    pub fn scale(
        source: Box<dyn NoiseFn<f64, 3>>,
        scale_settings: &ScaleSettings,
    ) -> Box<dyn NoiseFn<f64, 3>> {
        Box::new(
            ScaleBias::new(source)
                .set_bias(scale_settings.bias)
                .set_scale(scale_settings.scale),
        )
    }

    pub fn min(
        lhs: Box<dyn NoiseFn<f64, 3>>,
        rhs: Box<dyn NoiseFn<f64, 3>>,
    ) -> Box<dyn NoiseFn<f64, 3>> {
        Box::new(Min::new(lhs, rhs))
    }

    pub fn curve(
        source: Box<dyn NoiseFn<f64, 3>>,
        shape_settings: &TerrainShape,
    ) -> Box<dyn NoiseFn<f64, 3>> {
        if shape_settings.control_points.len() < 4 {
            panic!("You need at least four points for terrain shaping");
        }
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
            LayerNode::new(
                "fb1",
                LayerType::SimpleLayer(
                    LayerSettings::default()
                        .set_frequency(4.34375)
                        .set_octaves(11),
                ),
            ),
            LayerNode::new(
                "sb",
                LayerType::ScaleBias(ScaleSettings::new("fb1", 0.375, 0.625)),
            ),
            LayerNode::new("mi", LayerType::Min(MinSettings::new("sb", "cu"))),
            LayerNode::new("cl", LayerType::Clamp(ClampSettings::new("mi", -1.0, 1.0))),
        ]
    }
}
