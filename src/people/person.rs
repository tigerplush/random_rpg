use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub gender: Gender,
    pub name: String,
    pub surname: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Nonbinary,
}
