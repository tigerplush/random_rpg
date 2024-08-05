//! A procedural world generation library
//!
//! RandomRpg is meant to create immersive worlds and run simulations in them,
//! showing the user output in form of a wikipedia they can browse through.
//!
//! Interaction with this world is done via event files in the event folder.
//!
//! ## Usage
//! Call `rrpg init` to create a new world with default settings files. After
//! adjusting the files to your liking, call `rrpg generate` to create a world file
//! and it's wikipedia.
//!
//! Change the event file template to your liking and run `rrpg update` to incorporate
//! the specified event into the world.
use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

pub mod world;

use world::*;

pub mod utilities;

const DEFAULT_WORLD_NAME: &str = "unnamed_world";
const DEFAULT_SETTINGS_PATH: &str = "settings";
const DEFAULT_OUTPUT_PATH: &str = "output";
const DEFAULT_WORLD_SETTINGS_FILE: &str = "world_settings.yml";
const DEFAULT_MAP_SETTINGS_FILE: &str = "map_settings.yml";

pub fn init(name: Option<&String>, seed: Option<&u32>) -> Result<(), Box<dyn Error>> {
    let path = Path::new(".")
        .join(name.unwrap_or(&String::from(DEFAULT_WORLD_NAME)))
        .join(DEFAULT_SETTINGS_PATH);
    if !Path::exists(&path) {
        fs::create_dir_all(&path)?;
    }

    let settings_path = Path::new(&path).join(DEFAULT_WORLD_SETTINGS_FILE);
    let file = File::create(settings_path)?;
    serde_yaml::to_writer(file, &WorldSettings::from(seed))?;

    let settings_path = Path::new(&path).join(DEFAULT_MAP_SETTINGS_FILE);
    let file = File::create(settings_path)?;
    serde_yaml::to_writer(file, &MapSettings::default())?;

    Ok(())
}

pub fn generate(name: Option<&String>, debug: bool) -> Result<(), Box<dyn Error>> {
    let path = Path::new(".")
        .join(name.unwrap_or(&String::from(DEFAULT_WORLD_NAME)))
        .join(DEFAULT_SETTINGS_PATH);

    let settings_path = Path::new(&path).join(DEFAULT_WORLD_SETTINGS_FILE);
    let file = File::open(settings_path)?;
    let world_settings: WorldSettings = serde_yaml::from_reader(file)?;

    let settings_path = Path::new(&path).join(DEFAULT_MAP_SETTINGS_FILE);
    let file = File::open(settings_path)?;
    let map_settings: MapSettings = serde_yaml::from_reader(file)?;

    let output_path = Path::new(".")
        .join(name.unwrap_or(&String::from(DEFAULT_WORLD_NAME)))
        .join(DEFAULT_OUTPUT_PATH);
    if !Path::exists(&output_path) {
        fs::create_dir_all(&output_path)?;
    }
    let world = map::generate(world_settings.seed, &map_settings, output_path, debug);

    Ok(())
}
