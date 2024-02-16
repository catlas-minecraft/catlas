use std::collections::HashMap;
use std::{hash, result};
use std::{
    fs::File,
    path::Path
};

use fastanvil::Region;
use rustc_hash::FxHasher;

use crate::manager::SectionManager;

use crate::manager::error::ManagerError;
use crate::models::Chunk;

use super::{Result, XZCoord};
type Hasher = hash::BuildHasherDefault<FxHasher>;

pub struct ChunkManager {
    region: Region<File>,
    chunks: HashMap<(u8, u8), Option<SectionManager>, Hasher>
}

impl ChunkManager {
    pub const CHUNK_IN_SIDE: u16 = 32;
    pub const CHUNK_SIDE_BLOCK: u16 = 16;

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Option<ChunkManager>> {
        let path = path.as_ref();

        if path.is_file() {
            let file = File::open(path).map_err(ManagerError::FailedOpenRegion)?;
            let region = Region::from_stream(file).map_err(ManagerError::FailedLoadRegion)?;

            Ok(Option::Some(ChunkManager {
                chunks: HashMap::default(),
                region
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_chunk_by_region_chunk(&mut self, x: u8, z: u8) -> &mut Option<SectionManager> {
        self.chunks.entry((x, z)).or_insert_with(|| {
            let chunk: Option<Chunk> = match self.region.read_chunk(x as usize, z as usize) {
                Ok(raw_chunk) => {
                    if let Some(raw_chunk) = raw_chunk {
                        fastnbt::from_bytes(&raw_chunk).ok()
                    } else { None }
                },
                Err(err) => {
                    println!("[WARN] Failed read chunk:\n{:?}", err);
                    None
                }
            };

            if let Some(chunk) = chunk {
                Some(SectionManager::new(chunk))
            } else {
                None
            }
        })
    }

    pub fn get_chunk_by_chunk(&mut self, chunk_xz: XZCoord) -> &mut Option<SectionManager> {
        // もっと良い計算方法ありそう
        let region_chunk_x = (((chunk_xz.x % 32) + 32) % 32) as u8;
        let region_chunk_z = (((chunk_xz.z % 32) + 32) % 32) as u8;

        self.get_chunk_by_region_chunk(region_chunk_x, region_chunk_z)
    }

    pub fn get_chunk_xz_by_block_xz(block: &XZCoord) -> XZCoord {
        let chunk_x = block.x / (ChunkManager::CHUNK_SIDE_BLOCK as i32);
        let chunk_z = block.z / (ChunkManager::CHUNK_SIDE_BLOCK as i32);

        (chunk_x, chunk_z).into()
    }

    pub fn get_chunk_by_block(&mut self, block: &XZCoord) -> &mut Option<SectionManager> {
        let chunk_xz = ChunkManager::get_chunk_xz_by_block_xz(block);
        self.get_chunk_by_chunk(chunk_xz)
    }
}
