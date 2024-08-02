use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

mod world;

use world::*;

mod utilities;

pub fn init(name: Option<&String>, seed: Option<&u32>) -> Result<(), Box<dyn Error>> {
    let path = Path::new("./").join(name.unwrap_or(&String::from("unnamed_world")));
    if !Path::exists(&path) {
        fs::create_dir(&path)?;
    }

    let settings_path = Path::new(&path).join("world_settings.yml");
    let file = File::create(settings_path)?;
    serde_yaml::to_writer(file, &WorldSettings::from(seed))?;

    let settings_path = Path::new(&path).join("map_settings.yml");
    let file = File::create(settings_path)?;
    serde_yaml::to_writer(file, &MapSettings::default())?;


    Ok(())
}