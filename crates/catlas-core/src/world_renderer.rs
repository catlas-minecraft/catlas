use std::{
    fs::{self, File},
    io,
    iter::Peekable,
    path::{Path, PathBuf},
};

use catlas_colors::MapColor;
use catlas_models::Section;
use catlas_renderer::{REGION_SIZE, Render};
use fastanvil::Region;
use regex::Regex;

pub struct WorldRenderer {
    regions_iter: Peekable<std::vec::IntoIter<(i32, i32)>>,
    world_path: PathBuf,
    north_top_coords: [[i32; Section::SIZE as usize]; REGION_SIZE],
}

impl WorldRenderer {
    pub fn new<P: AsRef<Path>>(world_path: P) -> io::Result<WorldRenderer> {
        let region_file_regex = Regex::new(r"^r\.(?<x>-?\d+)\.(?<z>-?\d+).mca$").unwrap();
        let world_path = world_path.as_ref();

        let mut regions: Vec<(i32, i32)> = fs::read_dir(world_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if !entry.file_type().ok()?.is_file() {
                    return None;
                }

                let file_name: String = entry.file_name().to_string_lossy().into_owned();
                let captured = region_file_regex.captures(&file_name)?;

                let x: i32 = captured["x"].parse().ok()?;
                let z: i32 = captured["z"].parse().ok()?;

                Some((x, z))
            })
            .collect();

        regions.sort();

        let regions_iter = regions.into_iter().peekable();
        let north_top_coords = [[0; Section::SIZE as usize]; REGION_SIZE];

        Ok(WorldRenderer {
            regions_iter,
            world_path: world_path.into(),
            north_top_coords,
        })
    }
}

impl Iterator for WorldRenderer {
    type Item = WorldRenderResult;

    fn next(&mut self) -> Option<Self::Item> {
        let (region_x, region_z) = self.regions_iter.next()?;

        let region = File::open(
            self.world_path
                .join(format!("r.{}.{}.mca", region_x, region_z)),
        )
        .unwrap();
        let mut region = Region::from_stream(region).unwrap();
        // println!("{region_x}, {region_z}");

        let map = region.render(&mut self.north_top_coords).unwrap();

        if let Some((next_region_x, next_region_z)) = self.regions_iter.peek() {
            if region_x != *next_region_x || region_z != (next_region_z - 1) {
                self.north_top_coords = [[0; Section::SIZE as usize]; REGION_SIZE];
            }
        };

        return Some(WorldRenderResult {
            map,
            x: region_x,
            z: region_z,
        });
    }
}

pub struct WorldRenderResult {
    pub map: Vec<MapColor>,
    pub x: i32,
    pub z: i32,
}
