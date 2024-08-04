use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, 255]
    }
}

#[derive(Serialize, Deserialize)]
pub struct GradientPoint {
    pub position: f64,
    pub color: Color,
}

impl GradientPoint {
    pub const fn new(position: f64, r: u8, g: u8, b: u8) -> Self {
        GradientPoint {
            position,
            color: Color { r, g, b },
        }
    }
}
