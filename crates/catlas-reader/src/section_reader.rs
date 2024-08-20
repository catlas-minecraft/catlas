pub mod block_states_reader;

use std::iter::Once;

pub use block_states_reader::BlockStatesReader;
use block_states_reader::FullBlockStateYDirectionIter;
use catlas_models::Section;

use crate::y_pos::YPosItem;

#[derive(Debug)]
pub struct SectionReader {
    pub y: i8,
    pub block_states_reader: Option<BlockStatesReader>
}

impl SectionReader {
    pub fn new(section: Section) -> SectionReader {
        let block_states_reader = section.block_states.map(Into::into);

        SectionReader {
            y: section.y,
            block_states_reader
        }
    }

    pub fn y_direction_iter<'a>(&'a self, x: u8, z: u8) -> Option<SectionYDirectionIter<'a>> {
        Some(match self.block_states_reader.as_ref()? {
            BlockStatesReader::Full(state) => {
                SectionYDirectionIter::Full(self.y, state.y_direction_iter(x, z))
            },
            BlockStatesReader::Single(state) => {
                SectionYDirectionIter::Single(std::iter::once(state.get_sect_y_item().to_y_pos_item(self.y)))
            },
        })
    }
}

impl From<Section> for SectionReader {
    fn from(value: Section) -> Self {
        SectionReader::new(value)
    }
}

pub enum SectionYDirectionIter<'a> {
    Full(i8, FullBlockStateYDirectionIter<'a>),
    Single(Once<YPosItem<'a>>)
}

impl<'a> Iterator for SectionYDirectionIter<'a> {
    type Item = YPosItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SectionYDirectionIter::Full(section_y, iter) => Some(iter.next()?.to_y_pos_item(*section_y)),
            SectionYDirectionIter::Single(iter) => iter.next(),
        }
    }
}
