use std::{collections::HashMap, hash, path::PathBuf};

use rustc_hash::FxHasher;

use crate::manager::ChunkManager;

use super::{SectionManager, XZCoord};

type Hasher = hash::BuildHasherDefault<FxHasher>;

pub struct RegionManager {
    path: PathBuf,
    regions: HashMap<(i32, i32), Option<ChunkManager>, Hasher>
}

impl RegionManager {
    pub const REGION_SIDE_BLOCK: u16 = ChunkManager::CHUNK_SIDE_BLOCK * ChunkManager::CHUNK_IN_SIDE;

    pub fn new(path: PathBuf) -> RegionManager {
        let regions = HashMap::default();

        RegionManager {
            path,
            regions
        }
    }

    pub fn get_region_file_path(&self, region_coord: &XZCoord) -> PathBuf {
        self.path.join(format!("r.{}.{}.mca", region_coord.x, region_coord.z))
    }

    pub fn get_region_by_region_coord(&mut self, region_coord: XZCoord) -> &mut Option<ChunkManager> {
        let path = self.get_region_file_path(&region_coord);

        self.regions.entry((region_coord.x, region_coord.z)).or_insert_with(|| {
            let chunk_manager = ChunkManager::load(path).ok()?;
            chunk_manager
        })
    }


    pub fn get_region_coord_by_block_coord(block: &XZCoord) -> XZCoord {
        let region_x = block.x / (RegionManager::REGION_SIDE_BLOCK as i32);
        let region_z = block.z / (RegionManager::REGION_SIDE_BLOCK as i32);
        (region_x, region_z).into()
    }

    pub fn get_region_by_block(&mut self, block_coord: &XZCoord) -> &mut Option<ChunkManager> {
        let region_coord = RegionManager::get_region_coord_by_block_coord(block_coord);

        self.get_region_by_region_coord(region_coord)
    }

    pub fn get_region_by_chunk_coord(&mut self, chunk_coord: &XZCoord) -> &mut Option<ChunkManager> {
        let region_x = chunk_coord.x >> 5;
        let region_z = chunk_coord.z >> 5;

        self.get_region_by_region_coord((region_x, region_z).into())
    }

    pub fn get_chunk_by_chunk_coord(&mut self, chunk_coord: &XZCoord) -> Option<&mut SectionManager> {
        let mut region = self.get_region_by_chunk_coord(chunk_coord);
        match region {
            Some(region) => region.get_chunk_by_chunk_coord(chunk_coord).as_mut(),
            None => None
        }
    }

    pub fn get_chunk_by_block_coord(&mut self, block_coord: &XZCoord) -> Option<&mut SectionManager> {
        let region = match self.get_region_by_block(&block_coord) {
            Some(region) => region,
            None => return None
        };

        region.get_chunk_by_block(&block_coord).as_mut()
    }
}
