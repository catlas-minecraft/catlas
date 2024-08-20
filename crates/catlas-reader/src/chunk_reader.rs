use catlas_models::Chunk;

use crate::SectionReader;

pub struct ChunkReader {
    pub sections: Vec<SectionReader>,

    pub x_pos: i32,
    pub y_pos: i32,
    pub z_pos: i32
}

impl ChunkReader {
    pub fn new(chunk: Chunk) -> ChunkReader {
        ChunkReader {
            sections: chunk.sections.into_iter().map(Into::into).collect(),
            x_pos: chunk.x_pos,
            y_pos: chunk.y_pos,
            z_pos: chunk.z_pos
        }
    }
}

impl From<Chunk> for ChunkReader {
    fn from(value: Chunk) -> Self {
        ChunkReader::new(value)
    }
}
