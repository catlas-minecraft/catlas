
use std::time;
use std::fs::File;

use catlas_models::Section;
use catlas_renderer::Render;
use fastanvil::Region;
use image::{ImageFormat, RgbaImage};

fn main() {
    let now = time::Instant::now();

    let region = File::open("./chunks/r.-1.0.my.mca").unwrap();
    let mut region = Region::from_stream(region).unwrap();

    let mut north_y_coords = [[0; Section::SIZE as usize]; 32];
    let map = region.render(&mut north_y_coords).unwrap();

    let mut img = RgbaImage::new(Section::SIZE as u32 * 32, Section::SIZE as u32 * 32);

    for (dot, pixel) in map.iter().zip(img.pixels_mut()) {
        *pixel = image::Rgba(dot.rgba());
    }

    println!("{:?}", now.elapsed());

    img.save_with_format("result.png", ImageFormat::Png).unwrap();
    img.save_with_format("result.webp", ImageFormat::WebP).unwrap();
    img.save_with_format("result.gif", ImageFormat::Gif).unwrap();
}
