use std::{
    fs,
    path::{Path, PathBuf},
    time,
};

use catlas_models::Section;
use clap::{Args, ValueEnum};
use image::{ImageFormat, RgbaImage};

const SLIPPY_ZOOM: u8 = 17;
const SLIPPY_REAL_ORIGIN_TILE: i64 = 1_i64 << (SLIPPY_ZOOM - 1);

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputMode {
    Region,
    Slippy,
}

#[derive(Debug, Args)]
pub struct RenderOption {
    /// Output directory path
    #[arg(short, long, default_value = "./out/")]
    out: PathBuf,
    /// Output coordinate mode
    #[arg(long, value_enum, default_value_t = OutputMode::Region)]
    output_mode: OutputMode,
    path: PathBuf,
}

pub fn render(option: RenderOption) {
    let region_renderer = catlas_core::WorldRenderer::new(option.path).unwrap();

    let now = time::Instant::now();
    println!("Start rendering...");

    for render_result in region_renderer {
        let mut img = RgbaImage::new(Section::SIZE as u32 * 32, Section::SIZE as u32 * 32);

        for (dot_color, pixel) in render_result.map.into_iter().zip(img.pixels_mut()) {
            *pixel = image::Rgba(dot_color.into());
        }

        match option.output_mode {
            OutputMode::Region => {
                save_region(&img, &option.out, render_result.x, render_result.z);
                println!("Rendered region {}, {}", render_result.x, render_result.z);
            }
            OutputMode::Slippy => {
                save_slippy(&img, &option.out, render_result.x, render_result.z);
                println!(
                    "Rendered slippy tiles for region {}, {}",
                    render_result.x, render_result.z
                );
            }
        }
    }

    println!("Render time: {:?}", now.elapsed());
}

fn save_region(img: &RgbaImage, out: &Path, region_x: i32, region_z: i32) {
    let out_path = out.join(format!("{}.{}.gif", region_x, region_z));
    img.save_with_format(out_path, ImageFormat::Gif).unwrap();
}

fn save_slippy(img: &RgbaImage, out: &Path, region_x: i32, region_z: i32) {
    let tile_width = img.width() / 2;
    let tile_height = img.height() / 2;
    let origin = SLIPPY_REAL_ORIGIN_TILE;

    for tile_z in 0..2 {
        for tile_x in 0..2 {
            let x = tile_x * tile_width;
            let y = tile_z * tile_height;
            let tile = image::imageops::crop_imm(img, x, y, tile_width, tile_height).to_image();

            let slippy_x = origin + i64::from(region_x) * 2 + i64::from(tile_x);
            let slippy_y = origin + i64::from(region_z) * 2 + i64::from(tile_z);
            let out_dir = out.join(SLIPPY_ZOOM.to_string()).join(slippy_x.to_string());

            fs::create_dir_all(&out_dir).unwrap();

            let out_path = out_dir.join(format!("{}.gif", slippy_y));
            tile.save_with_format(out_path, ImageFormat::Gif).unwrap();
        }
    }
}
