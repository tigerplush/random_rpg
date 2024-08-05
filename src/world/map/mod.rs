pub mod clamp_settings;
pub mod color_gradient;
pub mod layer_node;
pub mod layer_settings;
pub mod map_settings;
pub mod min_settings;
pub mod scale_settings;
pub mod terrain_shape;

use std::path::PathBuf;

use map_settings::MapSettings;
use noise::utils::{
    ColorGradient, ImageRenderer, NoiseMapBuilder, PlaneMapBuilder, SphereMapBuilder,
};

use crate::utilities::UVec2;

pub struct World {
    tiles: Vec<(usize, usize, f64)>,
    size: UVec2,
}

pub fn generate(seed: u32, settings: &MapSettings, path: PathBuf, debug: bool) -> World {
    let result = settings.generate(seed, &path, debug);

    let mut color_gradient = ColorGradient::new();
    for gradient_point in &settings.color_gradient {
        color_gradient = color_gradient
            .add_gradient_point(gradient_point.position, gradient_point.color.to_array());
    }

    let noise_map = PlaneMapBuilder::new(&result)
        .set_size(settings.size.x, settings.size.y)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build();

    let image_renderer = ImageRenderer::new()
        .set_gradient(color_gradient.clone())
        .render(&noise_map);
    image_renderer.write_to_file(&path.join("map.png"));

    let sphere_map = &SphereMapBuilder::new(&result)
        .set_size(settings.size.x, settings.size.y)
        .set_bounds(-90.0, 90.0, -180.0, 180.0)
        .build();
    let image_renderer = ImageRenderer::new()
        .set_gradient(color_gradient.clone())
        .render(sphere_map);
    image_renderer.write_to_file(&path.join("map_sphere.png"));

    let mut values = Vec::new();
    for x in 0..sphere_map.size().0 {
        for y in 0..sphere_map.size().1 {
            values.push((x, y, sphere_map.get_value(x, y)));
        }
    }

    World {
        tiles: values,
        size: settings.size.clone(),
    }
}
