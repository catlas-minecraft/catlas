use catlas_models::Section;
use image::{ImageFormat, RgbaImage};

fn main() {
    let region_renderer = catlas_core::WorldRenderer::new("./chunks/").unwrap();

    for render_result in region_renderer {
        let mut img = RgbaImage::new(Section::SIZE as u32 * 32, Section::SIZE as u32 * 32);

        for (dot_color, pixel) in render_result.map.into_iter().zip(img.pixels_mut()) {
            *pixel = image::Rgba(dot_color.into());
        }

        img.save_with_format(
            format!("out/{}.{}.gif", render_result.x, render_result.z),
            ImageFormat::Gif,
        )
        .unwrap();
    }
}
