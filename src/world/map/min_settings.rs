use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MinSettings {
    pub lhs: String,
    pub rhs: String,
}

impl MinSettings {
    pub fn new(lhs: &str, rhs: &str) -> Self {
        MinSettings {
            lhs: String::from(lhs),
            rhs: String::from(rhs),
        }
    }
}
