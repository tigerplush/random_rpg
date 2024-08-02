use rand::{thread_rng, RngCore};
use serde::Serialize;

#[derive(Serialize)]
pub struct WorldSettings {
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
