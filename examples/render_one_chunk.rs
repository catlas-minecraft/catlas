
use std::fs::File;

use catlas::{models::Chunk, renderer::render_chunk};
use fastanvil::Region;

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

    img.save("result.png").unwrap();

    println!("complete")
}
