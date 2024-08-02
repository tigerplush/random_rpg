use std::{
    error::Error, fs::{self, File}, path::Path
};

use rand::{thread_rng, RngCore};
use serde::Serialize;

pub fn init() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./unnamed_world");
    if !Path::exists(path) {
        fs::create_dir(path)?;
    }
    let settings_path = Path::new(path).join("world_settings.yml");
    let file = File::create(settings_path)?;
    serde_yaml::to_writer(file, &WorldSettings::default())?;
    Ok(())
}

#[derive(Serialize)]
struct WorldSettings {
    seed: u32,
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            seed: thread_rng().next_u32(),
        }
    }
}
