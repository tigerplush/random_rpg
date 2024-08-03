pub mod layer_settings;
pub mod map_settings;
pub mod terrain_shape;

use core::panic;
use std::path::PathBuf;

use map_settings::MapSettings;
use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    BasicMulti, Curve, MultiFractal, Simplex,
};

pub fn generate(seed: u32, settings: &MapSettings, path: PathBuf) {
    let layers = settings.get_layers();
    if layers.is_empty() {
        panic!("You need to specify at least one layer");
    }
    for (i, layer) in layers.iter().enumerate() {
        let basic_multi = BasicMulti::<Simplex>::new(seed)
            .set_octaves(layer.octaves)
            .set_lacunarity(layer.lacunarity)
            .set_frequency(layer.frequency)
            .set_persistence(layer.persistence);
        if layer.terrain_shaping.control_points.len() < 4 {
            panic!("You need at least four points for terrain shaping");
        }
        let mut curve = Curve::new(basic_multi);
        for point in &layer.terrain_shaping.control_points {
            curve = curve.add_control_point(point.x, point.y);
        }
        let noise_map = PlaneMapBuilder::new(curve)
            .set_size(settings.get_world_width(), settings.get_world_height())
            .build();
        noise_map.write_to_file(&path.join(format!("map_{}.png", i)));
    }
}
