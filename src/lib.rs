use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

mod world;

use world::*;

mod utilities;

const DEFAULT_NAME: &str = "unnamed_world";
const DEFAULT_SETTINGS_PATH: &str = "settings";

pub fn init(name: Option<&String>, seed: Option<&u32>) -> Result<(), Box<dyn Error>> {
    let path = Path::new("./")
        .join(name.unwrap_or(&String::from(DEFAULT_NAME)))
        .join(DEFAULT_SETTINGS_PATH);
    if !Path::exists(&path) {
        fs::create_dir_all(&path)?;
    }

    let settings_path = Path::new(&path).join("world_settings.yml");
    let file = File::create(settings_path)?;
    serde_yaml::to_writer(file, &WorldSettings::from(seed))?;

    let settings_path = Path::new(&path).join("map_settings.yml");
    let file = File::create(settings_path)?;
    serde_yaml::to_writer(file, &MapSettings::default())?;

    Ok(())
}

pub fn generate(name: Option<&String>) -> Result<(), Box<dyn Error>> {
    let path = Path::new("./")
        .join(name.unwrap_or(&String::from(DEFAULT_NAME)))
        .join(DEFAULT_SETTINGS_PATH);

    let settings_path = Path::new(&path).join("world_settings.yml");
    let file = File::open(settings_path)?;
    let x: WorldSettings = serde_yaml::from_reader(file)?;
    println!("{}", x.get_seed());

    Ok(())
}
