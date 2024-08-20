// use std::ops::Deref;

// use catlas_colors::{BaseColor, BaseColorType, MapColor, BASE_COLOR_MAP};
// use catlas_models::Section;
// use catlas_reader::{BlockStatesReader, ChunkReader, SectionReader, YDirectionChunkIter};

// type SouthYCoordsOfNorthChunk = [i32; Section::SIZE as usize];

// pub trait TileRenderer {
//     fn render(
//         &self,
//         north_chunk_south_ys: &mut SouthYCoordsOfNorthChunk
//     ) -> Vec<MapColor>;

//     fn render_top(
//         x: u8,
//         z: u8,
//         north_y: i32
//     ) -> MapColor;
// }

// impl TileRenderer for ChunkReader {
//     fn render(&self, chunk_south_y_coords: &mut SouthYCoordsOfNorthChunk) -> Vec<MapColor> {
//         let sections: Vec<&SectionReader> = self.sections.iter().filter(|section| {
//             match section.block_states_reader {
//                 Some(reader) => match reader {
//                     BlockStatesReader::Full(reader) => true,
//                     BlockStatesReader::Single(reader) => {
//                         let base_color = BASE_COLOR_MAP.get(&reader.get_block().name);
//                         match base_color {
//                             Some(base_color) => !base_color.kind.is_none(),
//                             None => false
//                         }
//                     },
//                 }
//                 None => false
//             }
//         }).collect();
//     }

//     fn render_top(x: u8, z: u8, north_y: i32) -> MapColor {
        
//     }
// }

// pub struct ChunkRendererTask<'a> {
//     pub chunk_reader: &'a ChunkReader,
//     visible_block_sections: Vec<&'a SectionReader>
// }

// impl<'a> ChunkRendererTask<'a> {
//     pub fn new(chunk_reader: &'a ChunkReader) -> ChunkRendererTask<'a> {
//         let visible_block_sections: Vec<&'a SectionReader> = chunk_reader.sections.iter().filter(|section| {
//             match &section.block_states_reader {
//                 Some(reader) => match reader {
//                     BlockStatesReader::Full(_) => true,
//                     BlockStatesReader::Single(reader) => {
//                         let base_color = BASE_COLOR_MAP.get(&reader.get_block().name);
//                         match base_color {
//                             Some(base_color) => !base_color.kind.is_none(),
//                             None => false
//                         }
//                     },
//                 }
//                 None => false
//             }
//         }).collect();

//         ChunkRendererTask {
//             chunk_reader,
//             visible_block_sections
//         }
//     }

//     pub fn render(&self, chunk_south_y_coords: &mut SouthYCoordsOfNorthChunk) -> Vec<MapColor> {
//         let mut map = Vec::new();

//         for z in 0..Section::SIZE {
//             for x in 0..Section::SIZE {
//                 let north_y = chunk_south_y_coords[x as usize];

//                 let render_result = self.render_xz(x, z, north_y);

//                 let (map_color, y) = match render_result {
//                     Some(render_result) => (render_result.map_color, render_result.real_y),
//                     None => (MapColor::default(), -65)
//                 };

//                 chunk_south_y_coords[x as usize] = y;
//                 map.push(map_color);
//             }
//         }

//         map
//     }

//     pub fn render_xz(&self, x: u8, z: u8, north_y: i32) -> (MapColor, y) {
//         let base_color: Option<BaseColor> = match self.kind {
//             BaseColorType::Normal(base_color) => Some(base_color), 
//             BaseColorType::Bed(base_color) => match properties {
//                 Some(properties) => match properties.get("part") {
//                     Some(part) => match part.as_str() {
//                         "head" => Some(BaseColor::Wool),
//                         "foot" => Some(base_color),
//                         _ => None
//                     },
//                     None => None
//                 },
//                 None => None
//             },
//             BaseColorType::Axis(y, other) => match properties {
//                 Some(properties) => match properties.get("axis") {
//                     Some(axis) => match axis.as_str() {
//                         "y" => Some(y),
//                         "x" | "z" => Some(other),
//                         _ => None
//                     },
//                     None => None
//                 }
//                 None => None
//             }
//         };

//         let map_color = match base_color {
//             Some(base_color) => {
//                 if y == north_y {
//                     MapColor::new(base_color, MapColorLevel::Normal)
//                 } else if y > north_y {
//                     MapColor::new(base_color, MapColorLevel::Light)
//                 } else if y < north_y {
//                     MapColor::new(base_color, MapColorLevel::Dark)
//                 } else {
//                     unreachable!()
//                 }
//             },
//             None => MapColor::error()
//         };

//         map_color
//     }
// }

use std::cmp::Ordering;

use catlas_colors::{BaseColor, BaseColorType, MapColor, MapColorLevel};
use catlas_reader::SectionReader;

use crate::RenderPreTileResult;

pub trait RenderTile<'a> {
    fn render_tile(self, x: u8, z: u8, north_y: i32, render_pre_tile_result: Option<RenderPreTileResult<'a>>) -> MapColor;
}

impl<'a, II, I> RenderTile<'a> for II
where
    II: IntoIterator<Item = &'a SectionReader, IntoIter = I>,
    I: Iterator<Item = &'a SectionReader> + Sized + DoubleEndedIterator
{
    fn render_tile(self, x: u8, z: u8, north_y: i32, render_pre_tile_result: Option<RenderPreTileResult<'a>>) -> MapColor {
        let Some(render_pre_tile_result) = render_pre_tile_result else {
            return MapColor::none();
        };

        let (y_pos, paletted_block) = render_pre_tile_result.y_pos_item;
        let y = y_pos.real_y();

        let base_color: Option<BaseColor> = 'base: {
            match render_pre_tile_result.block_color.kind {
                BaseColorType::Normal(base_color) => Some(base_color),
                BaseColorType::Bed(base_color) => {
                    let Some(ref properties) = paletted_block.properties else { break 'base None };
                    let Some(part) = properties.get("part") else { break 'base None };

                    match part.as_str() {
                        "head" => Some(BaseColor::Wool),
                        "foot" => Some(base_color),
                        _ => None
                    }
                },
                BaseColorType::Axis(y, other) => {
                    let Some(ref properties) = paletted_block.properties else { break 'base None };
                    let Some(axis) = properties.get("axis") else { break 'base None };

                    match axis.as_str() {
                        "y" => Some(y),
                        "x" | "z" => Some(other),
                        _ => None
                    }
                }
            }
        };

        let map_color = match base_color {
            Some(base_color) => match y.cmp(&north_y) {
                Ordering::Equal => MapColor::new(base_color, MapColorLevel::Normal),
                Ordering::Greater => MapColor::new(base_color, MapColorLevel::Light),
                Ordering::Less => MapColor::new(base_color, MapColorLevel::Dark)
            }
            None => MapColor::error()
        };

        map_color
    }
}
