use crate::models::Section;

mod block_states_reader;

pub use block_states_reader::BlockStatesReader;

#[derive(Debug)]
pub struct SectionReader {
    pub block_states_reader: Option<BlockStatesReader>
}

impl<'a> SectionReader {
    pub fn new(section: Section) -> SectionReader {
        let block_states_reader = match section.block_states {
            Some(block_states) => Some(BlockStatesReader::new(block_states)),
            None => None
        };

        SectionReader {
            block_states_reader
        }
    }
}

impl From<Section> for SectionReader {
    fn from(value: Section) -> Self {
        SectionReader::new(value)
    }
}
