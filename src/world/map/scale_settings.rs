use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScaleSettings {
    pub input: String,
    pub scale: f64,
    pub bias: f64,
}

impl ScaleSettings {
    pub fn new(input: &str, scale: f64, bias: f64) -> Self {
        ScaleSettings {
            input: String::from(input),
            scale,
            bias,
        }
    }
}
