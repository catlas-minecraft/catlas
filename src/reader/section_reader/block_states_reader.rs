use std::cmp;

use crate::models::{FullBlockStates, SingleBlockStates, BlockStates, PalettedBlock};

#[derive(Debug)]
pub enum BlockStatesReader {
    FullBlockStatesReader(FullBlockStatesReader),
    SingleBlockStatesReader(SingleBlockStatesReader)
}

pub trait BlockStatesReaderTrait<'a> {
    fn get_block(x: u8, y: u8, z: u8) -> &'a PalettedBlock;
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
pub struct FullBlockStatesReader {
    base: FullBlockStates,
    data_bits: u32
}

impl FullBlockStatesReader {
    const MIN_BITS_PER_DATA: u32 = 4;

    /// 
    /// Returns the number of digits in binary for the number of pallets
    /// ```
    /// let palette_len: u8    = 0b00000111;
    /// let zeros          = palette_len.leadning_zeros(); // Return 5
    /// 
    /// assert!(u8::BITS - zeros == 3);
    /// ```
    /// 
    pub fn get_data_bits(palette_len: usize) -> u32 {
        cmp::max(
            FullBlockStatesReader::MIN_BITS_PER_DATA,
            usize::BITS - (palette_len).leading_zeros()
        )
    }

    /// 
    /// ```
    /// assert!(0b111 == SectionReader::to_bit_mask(3));
    /// ```
    /// 
    pub fn to_bit_mask(data_bits: u32) -> usize {
        (1 << data_bits) - 1
    }

    pub fn new(block_states: FullBlockStates) -> FullBlockStatesReader {
        let data_bits = FullBlockStatesReader::get_data_bits(block_states.palette.len());

        FullBlockStatesReader {
            base: block_states,
            data_bits
        }
    }
}

impl From<FullBlockStates> for FullBlockStatesReader {
    fn from(value: FullBlockStates) -> Self {
        FullBlockStatesReader::new(value)
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
}

impl From<SingleBlockStates> for SingleBlockStatesReader {
    fn from(value: SingleBlockStates) -> Self {
        SingleBlockStatesReader::new(value)
    }
}
