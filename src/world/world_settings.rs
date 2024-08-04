use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};

/// Describes the settings from which the world will be created
#[derive(Serialize, Deserialize)]
pub struct WorldSettings {
    /// Seed of all randomness
    pub seed: u32,
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
