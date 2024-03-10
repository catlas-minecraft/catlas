

use std::{collections::HashMap, hash::BuildHasherDefault, path::PathBuf};

use rustc_hash::FxHasher;

type Hasher = BuildHasherDefault<FxHasher>;

use crate::{
    manager::{
        self,
        RegionManager,
        XZCoord
    }, map_color::{self, BaseColorId}, models::PalettedBlock, reader::{BlockStatesReader, SectionReader}
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

    fn render_chunk(&self, x: i32, z: i32) {
        let mut chunk_renderer = ChunkRenderer::new(&self.region_manager);
    }
}

pub struct ChunkRenderer<'a> {
    manager: &'a RegionManager,
    sliced_chunks: HashMap<XZCoord, Option<Vec<&'a SectionReader>>, Hasher>,
    pub heighest_blocks: HashMap<XZCoord, Option<&'a PalettedBlock>, Hasher>
}

impl<'a> ChunkRenderer<'a> {
    pub fn new(manager: &'a RegionManager, ) -> ChunkRenderer<'a> {
        ChunkRenderer {
            manager,
            sliced_chunks: HashMap::default(),
            heighest_blocks: HashMap::default()
        }
    }

    pub fn get_sliced_chunks(&mut self, chunk_coord: XZCoord) -> Option<&mut Vec<&'a SectionReader>> {
        self.sliced_chunks.entry(chunk_coord.clone()).or_insert_with(|| {
            let chunk = self.manager.get_chunk_by_chunk_coord(&chunk_coord)?;

            let sliced_chunk: Vec<&'a SectionReader> = chunk.inner.iter().filter(|reader| {
                let Some(block_states_reader) = &reader.block_states_reader else {
                    return false;
                };

                match block_states_reader {
                    BlockStatesReader::FullBlockStatesReader(_) => true,
                    BlockStatesReader::SingleBlockStatesReader(single_block_states_reader) => {

                        match map_color::BASE_COLOR_ID_MAP.get(&single_block_states_reader.get_block().name) {
                            Some(color_map) => match color_map.0 {
                                BaseColorId::Normal(0) => false,
                                _ => true
                            }
                            // TODO: なかった場合はおかしいのでエラー処理を書く
                            None => false
                        }
                    }
                }
            }).collect();

            Some(sliced_chunk)
        }).as_mut()
    }

    pub fn get_top_block(&mut self, block_coord: &'a XZCoord) -> Option<&'a PalettedBlock> {
        self.heighest_blocks.entry(block_coord.clone()).or_insert_with(|| {
            self.manager.get_chunk_by_block_coord(block_coord);

            None
        });
    }
}
