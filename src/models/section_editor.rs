use std::cmp;
use thiserror::Error;

use crate::models::{
    chunk::{Section, SECTION_BLOCK_COUNT},
    PaletteCollector,
    PalettedBlock
};

#[derive(Debug, Error)]
pub enum SectionEditorError {
    #[error("Section.data count expected {0}, but got {1}")]
    SectionDataCountDoseNotMatch(usize, u32),
    #[error("Illegal palette index. got({0}) but pallete length is {1}")]
    IllegalPaletteIndex(i64, u64)
}

pub type SectionEditorResult<T> = Result<T, SectionEditorError>;

#[derive(Debug)]
pub struct SectionEditor {
    pub block_palette: PaletteCollector
}

impl SectionEditor {
    const MIN_PALETTE_BITS_LEN: u32 = 4;

    pub fn new() -> SectionEditor {
        SectionEditor {
            block_palette: PaletteCollector::new()
        }
    }

    pub fn from_section(section: &Section) -> SectionEditorResult<SectionEditor> {
        // match &section.block_states {
        //     Some(block_states) => {
        //         let palette_len = block_states.palette.len() as u64;
        //         // Always 4 bits if the type of block used is less than 16
        //         let palette_bits_length = cmp::max(
        //             SectionEditor::MIN_PALETTE_BITS_LEN,
        //             i64::BITS - (palette_len).leading_zeros()
        //         );
        //         let palette_bits = (1 << palette_bits_length) - 1;
        //         let block_per_long = i64::BITS / palette_bits_length;
        //         let section_data_count = (SECTION_BLOCK_COUNT as u32).div_ceil(block_per_long);

        //         println!("パレットのbit数: {}", palette_bits_length);
        //         println!("パレットのbit  : {:b}", palette_bits);
        //         println!("longに入るblock数: {}", block_per_long);
        //         println!("section dataの数: {}", section_data_count);

        //         let mut blocks: Vec<Block> = Vec::with_capacity(SECTION_BLOCK_COUNT);
        //         match &block_states.data {
        //             Some(section_item) => {
        //                 let data_length = section_item.len();

        //                 if (section_data_count as usize) != data_length {
        //                     return Err(SectionEditorError::SectionDataCountDoseNotMatch(data_length ,section_data_count));
        //                 }

        //                 for data in section_item.iter() {
        //                     for i in (0..(block_per_long * palette_bits_length)).step_by(palette_bits_length as usize) {
        //                         let palette_index = (data >> i) & palette_bits;

        //                         let palette_item = block_states.palette.get(palette_index as usize);

        //                         let palette_item = match palette_item {
        //                             Some(item) => item,
        //                             None => return  Err(SectionEditorError::IllegalPaletteIndex(palette_index, palette_len))
        //                         };

        //                         println!("{palette_index},{palette_item}");
        //                     }
        //                 }
        //             },
        //             None => {
                        
        //             }
        //         };
        //     }
        //     None => {

        //     }
        // };

        Ok(SectionEditor::new())
    }
}
