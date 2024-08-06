use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub gender: Gender,
    pub name: String,
    pub surname: String,
}

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Nonbinary,
}