use crate::{models::Chunk, reader::SectionReader};

#[derive(Debug)]
pub struct SectionManager {
    pub chunk: Chunk,
    pub inner: Vec<SectionReader>
}

impl SectionManager {
    pub fn new(chunk: Chunk) -> Self {
        SectionManager {
            chunk,
            inner: Default::default()
        }
    }
}
