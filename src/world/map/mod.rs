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

pub fn generate(seed: u32, settings: &MapSettings, path: PathBuf) {
    let result = settings.generate(seed, &path);

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

    let image_renderer = ImageRenderer::new()
        .set_gradient(color_gradient.clone())
        .render(
            &SphereMapBuilder::new(&result)
                .set_size(settings.size.x, settings.size.y)
                .set_bounds(-90.0, 90.0, -180.0, 180.0)
                .build(),
        );
    image_renderer.write_to_file(&path.join("map_sphere.png"))
}
