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

// pub enum CurrentIterSection<'a> {
//     Full(&'a SectionReader, YDirectionSectionIter<'a>),
//     Single(&'a SectionReader, &'a SingleBlockStatesReader)
// }

// impl<'a> CurrentIterSection<'a> {
//     pub fn from_section_reader(section_reader: &'a SectionReader, x: u8, z: u8) -> Option<CurrentIterSection> {
//         match &section_reader.block_states_reader {
//             Some(reader) => match reader {
//                 BlockStatesReader::Full(reader) => Some(CurrentIterSection::Full(&section_reader, reader.y_direction_iter(x, z))),
//                 BlockStatesReader::Single(reader) => Some(CurrentIterSection::Single(&section_reader, reader))
//             },
//             None => None
//         }
//     }
// }

// pub struct YDirectionChunkIter<'a, I: Iterator<Item = &'a SectionReader>> {
//     x: u8,
//     z: u8,
//     sect_iter: I,
//     current_section: Option<(&'a SectionReader, YDirectionSectionIter<'a>)>
// }

// impl<'a, I> YDirectionChunkIter<'a, I>
// where
//     I: Iterator<Item = &'a SectionReader>
// {
//     pub fn new(sect_iter: I, x: u8, z: u8) -> YDirectionChunkIter<'a, I>
//     {
//         YDirectionChunkIter {
//             x, z,
//             sect_iter,
//             current_section: None
//         }
//     }

    // pub fn next_current_section(&mut self) -> Option<SectYItem<'a>> {

    //     // if self.current_section.is_none() {
    //     //     self.current_section = loop {
    //     //         let current_section: &'a SectionReader = self.sect_iter.next()?;

    //     //         match current_section.block_states_reader().as_ref() {
    //     //             Some(reader) => match reader {
    //     //                 BlockStatesReader::Full(reader) => break Some(reader.y_direction_iter(self.x, self.z).into()),
    //     //                 BlockStatesReader::Single(reader) => break Some(reader.into())
    //     //             },
    //     //             None => continue
    //     //         };
    //     //     };
    //     // }

    // }

    // pub fn next_current_block(&mut self) -> Option<SectYItem<'a>> {
    //     let sect_y_item = loop {
    //         let current_section = self.next_current_section()?;

    //         match current_section {
    //             CurrentIterSection::Full(iter) => {
    //                 match iter.next() {
    //                     Some(item) => break Some(item),
    //                     None => continue
    //                 }
    //             },
    //             CurrentIterSection::Single(reader) => {
    //                 break Some(reader.get_sect_y_item())
    //             }
    //         }
    //     };

    //     sect_y_item
    // }
// }

// impl<'a, I> Iterator for YDirectionChunkIter<'a, I>
// where
//     I: Iterator<Item = &'a SectionReader>
// {
//     type Item = (YPos, &'a PalettedBlock);

//     fn next(&mut self) -> Option<Self::Item> {
//         let sect_y_item: Self::Item = loop {
//             let sect_y_item: Self::Item = match &mut self.current_section {
//                 Some((sect_reader, iter)) => match iter.next() {
//                     Some(sect_y_item) => {
//                         sect_y_item.to_y_pos_item(sect_reader.y)
//                     },
//                     None => {
//                         self.current_section = None;

//                         continue;
//                     }
//                 },
//                 None => {
//                     let sect_reader = self.sect_iter.next()?;

//                     match &sect_reader.block_states_reader {
//                         Some(reader) => match reader {
//                             BlockStatesReader::Full(reader) => {
//                                 self.current_section = Some((sect_reader, reader.y_direction_iter(self.x, self.z)));

//                                 continue;
//                             },
//                             BlockStatesReader::Single(reader) => {
//                                 reader.get_sect_y_item().to_y_pos_item(sect_reader.y)
//                             }
//                         }
//                         None => continue
//                     }
//                 }
//             };

//             break sect_y_item;
//         };

//         Some(sect_y_item)
//     }
// }
