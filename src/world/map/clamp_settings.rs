use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClampSettings {
    pub input: String,
    pub lower_bounds: f64,
    pub upper_bounds: f64,
}

impl ClampSettings {
    pub fn new(input: &str, lower_bounds: f64, upper_bounds: f64) -> Self {
        ClampSettings {
            input: String::from(input),
            lower_bounds,
            upper_bounds,
        }
    }
}
