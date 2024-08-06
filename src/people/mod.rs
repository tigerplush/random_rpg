mod person;
use person::Person;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct People {
    alive: Vec<Person>,
    dead: Vec<Person>,
}

pub fn generate() -> People {
    People {
        alive: Vec::new(),
        dead: Vec::new(),
    }
}
