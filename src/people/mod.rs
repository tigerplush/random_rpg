mod person;
use person::{Gender, Person};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::{Normal, StandardNormal};
use serde::{Deserialize, Serialize};

use crate::HumanNames;

#[derive(Serialize, Deserialize)]
pub struct People {
    alive: Vec<Person>,
    dead: Vec<Person>,
}

pub fn generate(seed: u32, names: &HumanNames) -> People {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut alive = Vec::new();
    for _n in 0..400 {
        let random_number: u8 = rng.gen_range(0..=2);
        let gender = match random_number {
            0 => Gender::Male,
            1 => Gender::Female,
            _ => Gender::Nonbinary,
        };
        let surname = names.surnames[rng.gen_range(0..names.surnames.len())].to_owned();
        let name = match gender {
            Gender::Male => names.male[rng.gen_range(0..names.male.len())].to_owned(),
            Gender::Female => names.female[rng.gen_range(0..names.female.len())].to_owned(),
            Gender::Nonbinary => {
                names.non_binary[rng.gen_range(0..names.non_binary.len())].to_owned()
            }
        };
        let normal = Normal::new(0.0, 23.0).unwrap();
        let age: f64 = rng.sample(normal);
        alive.push(Person {
            gender,
            name,
            surname,
            age: age.abs() as u8,
        });
    }
    People {
        alive,
        dead: Vec::new(),
    }
}
