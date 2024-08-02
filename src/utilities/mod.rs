use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UVec2 {
    pub x: usize,
    pub y: usize,
}

impl UVec2 {
    pub const fn new(x: usize, y: usize) -> Self {
        UVec2 { x, y }
    }
}
