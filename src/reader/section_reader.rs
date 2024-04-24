pub mod block_states_reader;

use crate::models::Section;

pub use block_states_reader::BlockStatesReader;

#[derive(Debug)]
pub struct SectionReader {
    pub y: i8,
    block_states_reader: Option<BlockStatesReader>
}

impl<'a> SectionReader {
    pub fn new(section: Section) -> SectionReader {
        let block_states_reader = section.block_states.map(Into::into);

        SectionReader {
            y: section.y,
            block_states_reader
        }
    }

    pub fn block_states_reader(&'a self) -> &'a Option<BlockStatesReader> {
        &self.block_states_reader
    }
}

impl From<Section> for SectionReader {
    fn from(value: Section) -> Self {
        SectionReader::new(value)
    }
}
