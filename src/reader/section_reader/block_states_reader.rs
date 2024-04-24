mod full_block_states_reader;

use crate::models::{
    BlockStates, PalettedBlock, SingleBlockStates
};

pub use full_block_states_reader::FullBlockStatesReader;

#[derive(Debug)]
pub enum BlockStatesReader {
    FullBlockStatesReader(FullBlockStatesReader),
    SingleBlockStatesReader(SingleBlockStatesReader)
}

impl BlockStatesReader {
    pub fn new(block_states: BlockStates) -> BlockStatesReader {
        match block_states {
            BlockStates::FullBlockStates(block_states) => BlockStatesReader::FullBlockStatesReader(block_states.into()),
            BlockStates::SingleBlockStates(block_states) => BlockStatesReader::SingleBlockStatesReader(block_states.into())
        }
    }
}

impl From<BlockStates> for BlockStatesReader {
    fn from(value: BlockStates) -> Self {
        BlockStatesReader::new(value)
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

    pub fn get_block(&self) -> &PalettedBlock {
        &self.base.palette[0]
    }
}

impl From<SingleBlockStates> for SingleBlockStatesReader {
    fn from(value: SingleBlockStates) -> Self {
        SingleBlockStatesReader::new(value)
    }
}
