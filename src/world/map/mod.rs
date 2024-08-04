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
use rand::{rngs::StdRng, RngCore, SeedableRng};

pub fn generate(seed: u32, settings: &MapSettings, path: PathBuf) {
    if settings.layers.is_empty() {
        panic!("You need to specify at least one layer");
    }
    let mut rng = StdRng::seed_from_u64(seed as u64);
    for (i, layer) in settings.layers.iter().enumerate() {
        let layer_seed = match layer.seed {
            Some(seed) => seed,
            None => rng.next_u32(),
        };
        println!("layer {} seed is {}", i, layer_seed);
        let basic_multi = BasicMulti::<Simplex>::new(layer_seed)
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
            .set_size(settings.size.x, settings.size.y)
            .build();
        noise_map.write_to_file(&path.join(format!("layer_{}.png", i)));
    }
}
