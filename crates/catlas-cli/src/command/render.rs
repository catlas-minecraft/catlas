use std::path::PathBuf;

use catlas_models::Section;
use clap::Args;
use image::{ImageFormat, RgbaImage};

#[derive(Debug, Args)]
pub struct RenderOption {
    /// Output directory path
    #[arg(short, long, default_value = "./out/")]
    out: PathBuf,
    path: PathBuf,
}

pub fn render(option: RenderOption) {
    let region_renderer = catlas_core::WorldRenderer::new(option.path).unwrap();

    for render_result in region_renderer {
        let mut img = RgbaImage::new(Section::SIZE as u32 * 32, Section::SIZE as u32 * 32);

        for (dot_color, pixel) in render_result.map.into_iter().zip(img.pixels_mut()) {
            *pixel = image::Rgba(dot_color.into());
        }

        let out_path = option
            .out
            .join(format!("{}.{}.gif", render_result.x, render_result.z));

        img.save_with_format(out_path, ImageFormat::Gif).unwrap();
    }
}
