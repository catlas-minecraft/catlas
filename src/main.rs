mod models;
mod map_color;
mod reader;
mod renderer;
mod manager;

use std::time::Instant;

use crate::{manager::{ChunkManager, RegionManager}, renderer::Renderer};

fn main() {
    let start = Instant::now();

    let mut manager = RegionManager::new("./chunks/region".into());
    // let a = ((-33 % 32) + 32) % 32;

    let chunk = manager.get_chunk_by_block_coord(&((512, 16).into())).expect("msg");
    
    println!("Time elapsed in by_secure_hash() is: {:?}", start.elapsed());
    // println!("{:?}", chunk);
}
