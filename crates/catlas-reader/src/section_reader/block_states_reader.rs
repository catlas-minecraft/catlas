mod full_block_states_reader;

use catlas_models::{BlockStates, PalettedBlock, SingleBlockStates};

pub use full_block_states_reader::*;

use crate::{YPos, YPosItem, YPosItemKind};

#[derive(Debug)]
pub enum BlockStatesReader {
    Full(FullBlockStatesReader),
    Single(SingleBlockStatesReader),
}

impl BlockStatesReader {
    pub fn new(block_states: BlockStates) -> BlockStatesReader {
        match block_states {
            BlockStates::FullBlockStates(block_states) => {
                BlockStatesReader::Full(block_states.into())
            }
            BlockStates::SingleBlockStates(block_states) => {
                BlockStatesReader::Single(block_states.into())
            }
        }
    }
}

impl From<BlockStates> for BlockStatesReader {
    fn from(value: BlockStates) -> Self {
        BlockStatesReader::new(value)
    }
}

pub enum SectYItem<'a> {
    Single {
        paletted_block: &'a PalettedBlock,
    },
    Full {
        y_in_section: u8,
        paletted_block: &'a PalettedBlock,
    },
}

impl<'a> SectYItem<'a> {
    pub fn to_y_pos_item(self, section_y: i8) -> YPosItem<'a> {
        match self {
            SectYItem::Single { paletted_block } => YPosItem {
                y_pos: YPos::new(section_y, 15),
                paletted_block,
                kind: YPosItemKind::Single,
            },
            SectYItem::Full {
                y_in_section,
                paletted_block,
            } => YPosItem {
                y_pos: YPos::new(section_y, y_in_section),
                paletted_block,
                kind: YPosItemKind::Full,
            },
        }
    }
}

#[derive(Debug)]
pub struct SingleBlockStatesReader {
    base: SingleBlockStates,
}

impl SingleBlockStatesReader {
    pub fn new(block_states: SingleBlockStates) -> SingleBlockStatesReader {
        SingleBlockStatesReader { base: block_states }
    }

    pub fn get_sect_y_item(&self) -> SectYItem {
        SectYItem::Single {
            paletted_block: self.get_block(),
        }
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
