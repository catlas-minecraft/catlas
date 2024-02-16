

use std::{collections::HashMap, path::PathBuf};

use crate::{
    manager::{
        self,
        RegionManager,
        XZCoord
    },
    models::{Chunk, PalettedBlock}
};

pub struct Renderer {
    region_manager: RegionManager
}

impl Renderer {
    pub fn new(path: PathBuf) -> Self {
        let region_manager = manager::RegionManager::new(path);

        Renderer {
            region_manager
        }
    }

    fn render_chunk(&mut self, x: i32, z: i32) {
        let chunk = self.region_manager.get_chunk_by_block(&((512, 16).into()));
    }

    fn render_region() {
        
    }
}

pub struct ChunkRenderer<'a> {
    manager: &'a RegionManager,
    cache: HashMap<XZCoord, &'a PalettedBlock>
}

impl<'a> ChunkRenderer<'a> {
    pub fn new(manager: &'a RegionManager) -> ChunkRenderer<'a> {
        ChunkRenderer {
            manager
        }
    }
}
