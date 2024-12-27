use std::fs::File;
use std::time;

use catlas_models::Section;
use catlas_renderer::Render;
use fastanvil::Region;
use image::{ImageFormat, RgbaImage};

fn main() {
    let now = time::Instant::now();
    let region = File::open("chunks/r.0.0.catlas.mca").unwrap();
    let mut region = Region::from_stream(region).unwrap();

    let mut north_y_coords = [[0; Section::SIZE as usize]; 32];
    let map = region.render(&mut north_y_coords).unwrap();

    let mut img = RgbaImage::new(Section::SIZE as u32 * 32, Section::SIZE as u32 * 32);

    for (dot_color, pixel) in map.into_iter().zip(img.pixels_mut()) {
        *pixel = image::Rgba(dot_color.into());
    }

    img.save_with_format("out/result.png", ImageFormat::Png)
        .unwrap();
    img.save_with_format("out/result.webp", ImageFormat::WebP)
        .unwrap();
    img.save_with_format("out/result.gif", ImageFormat::Gif)
        .unwrap();

    println!("{:?}", now.elapsed());
}
