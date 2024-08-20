mod full_block_states_reader;

use catlas_models::{
    BlockStates, PalettedBlock, SingleBlockStates
};

pub use full_block_states_reader::*;

use crate::YPos;

#[derive(Debug)]
pub enum BlockStatesReader {
    Full(FullBlockStatesReader),
    Single(SingleBlockStatesReader)
}

impl BlockStatesReader {
    pub fn new(block_states: BlockStates) -> BlockStatesReader {
        match block_states {
            BlockStates::FullBlockStates(block_states) => BlockStatesReader::Full(block_states.into()),
            BlockStates::SingleBlockStates(block_states) => BlockStatesReader::Single(block_states.into())
        }
    }
}

impl From<BlockStates> for BlockStatesReader {
    fn from(value: BlockStates) -> Self {
        BlockStatesReader::new(value)
    }
}

pub struct SectYItem<'a> {
    pub y_in_section: u8,
    pub paletted_block: &'a PalettedBlock
}

impl<'a> SectYItem<'a> {
    pub fn new(y_in_section: u8, paletted_block: &'a PalettedBlock) -> SectYItem<'a> {
        SectYItem {
            y_in_section,
            paletted_block
        }
    }

    pub fn to_y_pos_item(self, section_y: i8) -> (YPos, &'a PalettedBlock) {
        (YPos::new(section_y, self.y_in_section), self.paletted_block)
    }
}

#[derive(Debug)]
pub struct SingleBlockStatesReader {
    base: SingleBlockStates
}

impl SingleBlockStatesReader {
    pub fn new(block_states: SingleBlockStates) -> SingleBlockStatesReader {
        SingleBlockStatesReader {
            base: block_states
        }
    }

    pub fn get_sect_y_item(&self) -> SectYItem {
        SectYItem::new(15, self.get_block())
    }

    pub fn get_block(&self) -> &PalettedBlock {
        &self.base.palette[0]
    }
}

impl From<SingleBlockStates> for SingleBlockStatesReader {
    fn from(value: SingleBlockStates) -> Self {
        SingleBlockStatesReader::new(value)
    }
}
