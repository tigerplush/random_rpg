use serde::Serialize;

use crate::utilities::UVec2;

#[derive(Serialize)]
pub struct MapSettings {
    size: UVec2,
}

impl Default for MapSettings {
    fn default() -> Self {
        MapSettings {
            // this is roughly the size of the eart in km
            size: UVec2::new(40075, 20014),
        }
    }
}