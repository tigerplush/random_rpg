use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

use rand::{thread_rng, RngCore};
use serde::Serialize;

pub fn init(name: Option<&String>, seed: Option<&u32>) -> Result<(), Box<dyn Error>> {
    let path = Path::new("./").join(name.unwrap_or(&String::from("unnamed_world")));
    if !Path::exists(&path) {
        fs::create_dir(&path)?;
    }
    let settings_path = Path::new(&path).join("world_settings.yml");
    let file = File::create(settings_path)?;
    serde_yaml::to_writer(file, &WorldSettings::from(seed))?;
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

impl From<Option<&u32>> for WorldSettings {
    fn from(value: Option<&u32>) -> Self {
        match value {
            Some(seed) => WorldSettings { seed: *seed },
            None => WorldSettings::default(),
        }
    }
}
