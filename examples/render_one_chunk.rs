
use std::fs::File;

use catlas::renderer::render_chunk;
use catlas_models::Chunk;
use fastanvil::Region;
use image::ImageFormat;

fn main() {
    println!("start");
    let region = File::open("./chunks/r.0.0.mca").unwrap();
    let mut region = Region::from_stream(region).unwrap();

    let chunk = region.read_chunk(15, 15).unwrap().unwrap();

    let chunk: Chunk = fastnbt::from_bytes(&chunk).unwrap();

    let map = render_chunk(chunk);

    let mut img = image::RgbaImage::new(16, 16);

    for (dot, pixel) in map.iter().zip(img.pixels_mut()) {
        *pixel = image::Rgba(dot.rgba());
    }

    img.save_with_format("result.webp", ImageFormat::WebP).unwrap();
    println!("complete");
}
