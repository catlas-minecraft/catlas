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

    pub fn get_region_file_path(&self, region_xz: &XZCoord) -> PathBuf {
        self.path.join(format!("r.{}.{}.mca", region_xz.x, region_xz.z))
    }

    pub fn get_region_by_region(&mut self, region_coord: XZCoord) -> &mut Option<ChunkManager> {
        let path = self.get_region_file_path(&region_coord);

        self.regions.entry((region_coord.x, region_coord.z)).or_insert_with(|| {
            let chunk_manager = ChunkManager::load(path).ok()?;
            chunk_manager
        })
    }


    pub fn get_region_xz_by_block_xz(block: &XZCoord) -> XZCoord {
        let region_x = block.x / (RegionManager::REGION_SIDE_BLOCK as i32);
        let region_z = block.z / (RegionManager::REGION_SIDE_BLOCK as i32);
        (region_x, region_z).into()
    }

    pub fn get_region_by_block(&mut self, block_xz: &XZCoord) -> &mut Option<ChunkManager> {
        let region_xz = RegionManager::get_region_xz_by_block_xz(block_xz);

        self.get_region_by_region(region_xz)
    }

    pub fn get_chunk_by_block(&mut self, block_xz: &XZCoord) -> Option<&mut SectionManager> {
        let region = match self.get_region_by_block(&block_xz) {
            Some(region) => region,
            None => return None
        };

        region.get_chunk_by_block(&block_xz).as_mut()
    }
}
