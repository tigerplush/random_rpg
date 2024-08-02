pub mod map_settings;

use std::path::PathBuf;

use map_settings::MapSettings;
use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    BasicMulti, Simplex,
};

pub fn generate(seed: u32, settings: &MapSettings, path: PathBuf) {
    let basic_multi = BasicMulti::<Simplex>::new(seed);
    let noise_map = PlaneMapBuilder::new(&basic_multi)
        .set_size(settings.get_world_width(), settings.get_world_height())
        .build();
    noise_map.write_to_file(&path.join("map.png"));
}
