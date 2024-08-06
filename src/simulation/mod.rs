use serde::{Deserialize, Serialize};

use crate::{map::World, People};

#[derive(Serialize, Deserialize)]
pub struct Simulation {
    pub world: World,
    pub people: People,
}
