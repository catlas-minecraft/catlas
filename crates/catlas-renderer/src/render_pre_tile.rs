// use catlas_colors::{BlockColor, BASE_COLOR_MAP};
// use catlas_models::PalettedBlock;
// use catlas_reader::{block_states_reader::{FullBlockStatesReader, SectYItem, SingleBlockStatesReader}, BlockStatesReader, SectionReader, YPos};

// pub struct PreRenderResult<'a> {
//     y: YPos,
//     paletted_block: &'a PalettedBlock,
//     block_color: BlockColor
// }

// pub trait RendererPreTile<'a> {
//     fn get_top(&'a self, x: u8, z: u8) -> Option<PreRenderResult<'a>>;
// }

// impl<'a> RendererPreTile<'a> for SectionReader {
//     fn get_top(&'a self, x: u8, z: u8) -> Option<PreRenderResult<'a>> {
//         let Some(block_states_reader) = &self.block_states_reader else {
//             return None;
//         };

//         let render_result = match block_states_reader {
//             BlockStatesReader::Full(reader) => reader.get_top(x, z),
//             BlockStatesReader::Single(reader) => reader.get_top(x, z),
//         };

//         let render_result = match render_result {
//             Some(render_result) => {
//                 Some(render_result.into_pre_render_result(self.y))
//             },
//             None => None
//         };

//         render_result
//     }
// }

// pub struct PreTileBlockStatesRendererResult<'a> {
//     y_in_section: u8,
//     paletted_block: &'a PalettedBlock,
//     block_color: BlockColor
// }

// impl<'a> PreTileBlockStatesRendererResult<'a> {
//     pub fn into_pre_render_result(self, section_y: i8) -> PreRenderResult<'a> {
//         PreRenderResult {
//             y: YPos::new(section_y, self.y_in_section),
//             block_color: self.block_color,
//             paletted_block: self.paletted_block
//         }
//     }
// }

// pub trait PreBlockStatesTileRenderer<'a> {
//     fn get_top(&'a self, x: u8, z: u8) -> Option<PreTileBlockStatesRendererResult<'a>>;
// }

// impl<'a> PreBlockStatesTileRenderer<'a> for FullBlockStatesReader {
//     fn get_top(&'a self, x: u8, z: u8) -> Option<PreTileBlockStatesRendererResult<'a>> {
//         for SectYItem { y_in_section, paletted_block } in self.y_direction_iter(x, z) {
//             let block_color = BASE_COLOR_MAP.get(&paletted_block.name);

//             if let Some(block_color) = block_color {
//                 if !block_color.kind.is_none() {
//                     let render_result = PreTileBlockStatesRendererResult {
//                         paletted_block,
//                         block_color: *block_color,
//                         y_in_section
//                     };

//                     return Some(render_result);
//                 }
//             } else {
//                 println!("[WARN] Failed get block({})", paletted_block);
//             }
//         }

//         None
//     }
// }

// impl<'a> PreBlockStatesTileRenderer<'a> for SingleBlockStatesReader {
//     fn get_top(&'a self, x: u8, z: u8) -> Option<PreTileBlockStatesRendererResult<'a>> {
//         todo!()
//     }
// }

use catlas_colors::{BlockColor, BASE_COLOR_MAP};
use catlas_reader::{SectionReader, YPosItem};

#[derive(Debug)]
pub struct RenderPreTileResult<'a> {
    pub y_pos_item: YPosItem<'a>,
    pub block_color: BlockColor
}

pub trait RenderPreTile<'a> {
    fn render_pre_tile(self, x: u8, z: u8) -> Option<RenderPreTileResult<'a>>;
}

impl<'a, II, I> RenderPreTile<'a> for II
where
    II: IntoIterator<Item = &'a SectionReader, IntoIter = I>,
    I: Iterator<Item = &'a SectionReader> + Sized + DoubleEndedIterator
{
    fn render_pre_tile(self, x: u8, z: u8) -> Option<RenderPreTileResult<'a>> {
        let iter = IntoIterator::into_iter(self).rev().filter_map(|el| {
            el.y_direction_iter(x, z)
        }).flatten();

        for (y_pos, paletted_block) in iter {
            let block_color = BASE_COLOR_MAP.get(&paletted_block.name);

            let Some(block_color) = block_color else {
                // println!("[WARN] Failed get block({})", paletted_block);
                continue;
            };

            if !block_color.kind.is_none() {
                let render_result = RenderPreTileResult {
                    y_pos_item: (y_pos, &paletted_block),
                    block_color: *block_color
                };

                return Some(render_result);
            }
        }

        None
    }
}
