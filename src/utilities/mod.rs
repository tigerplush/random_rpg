use serde::Serialize;

#[derive(Serialize)]
pub struct UVec2 {
    x: u32,
    y: u32,
}

impl UVec2 {
    pub const fn new(x: u32, y: u32) -> Self {
        UVec2 {
            x,
            y,
        }
    }
}