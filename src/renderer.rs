// use catlas_models::{Chunk, Properties, Section};

// use catlas_colors::{BaseColor, BaseColorType, BlockColor, MapColor, MapColorLevel, BASE_COLOR_MAP};
// use catlas_reader::{block_states_reader::{FullBlockStatesReader, SingleBlockStatesReader}, BlockStatesReader, SectionReader};

// pub fn render_chunk(chunk: Chunk) -> Vec<MapColor> {
//     let mut non_none_sections: Vec<SectionReader> = Vec::new();

//     for section in chunk.sections.into_iter().rev() {
//         let reader = SectionReader::new(section);

//         let Some(block_states_reader) = reader.block_states_reader() else {
//             continue;
//         };

//         match &block_states_reader {
//             &BlockStatesReader::Full(_) => {
//                 non_none_sections.push(reader);
//             },
//             &BlockStatesReader::Single(state_reader) => {
//                 let paletted_block = state_reader.get_block();

//                 if let Some(map_color) = BASE_COLOR_MAP.get(&paletted_block.name) {
//                     if !map_color.kind.is_none() {
//                         non_none_sections.push(reader)
//                     }
//                 }
//             }
//         }
//     }

//     let mut map: Vec<MapColor> = Vec::with_capacity((Section::SIZE as usize) * (Section::SIZE as usize));

//     let mut north_ys: [i32; Section::SIZE as usize] = [0; Section::SIZE as usize];

//     for z in 0..Section::SIZE {
//         for x in 0..Section::SIZE {
//             let north_y = north_ys[x as usize];

//             let render_result = non_none_sections.render_xy(x, z, north_y);

//             let (map_color, y) = if let Some(render_result) = render_result {
//                 (render_result.map_color, render_result.real_y)
//             } else {
//                 (MapColor::default(), -64)
//             };

//             north_ys[x as usize] = y;
//             map.push(map_color);
//         }
//     }

//     map
// }

// pub struct RenderResult {
//     map_color: MapColor,
//     real_y: i32
// }

// impl RenderResult {
//     pub fn new(map_color: MapColor, real_y: i32) -> RenderResult {
//         RenderResult {
//             map_color,
//             real_y
//         }
//     }
// }

// trait Renderer {
//     fn render_xy(&self, x: u8, z: u8, north_y: i32) -> Option<RenderResult>;
// }

// impl Renderer for Vec<SectionReader> {
//     fn render_xy(&self, x: u8, z: u8, north_y: i32) -> Option<RenderResult> {
//         for section_reader in self {
//             let render_result = section_reader.render_xy(x, z, north_y);

//             if let Some(render_result) = render_result{
//                 return Some(render_result);
//             }
//         }

//         None
//     }
// }

// impl Renderer for SectionReader {
//     fn render_xy(&self, x: u8, z: u8, north_y: i32) -> Option<RenderResult> {
//         let Some(block_states_reader) = self.block_states_reader() else {
//             return None;
//         };

//         let render = match block_states_reader {
//             BlockStatesReader::Full(reader) => reader.render_xz(x, z, self.y, north_y),
//             BlockStatesReader::Single(reader) => reader.render_xz(x, z, self.y, north_y),
//         };

//         render
//     }
// }

// pub trait PreRenderer {
//     fn render_xz(&self, x: u8, z: u8, chunk_y: i8, north_y: i32) -> Option<RenderResult>;
// }

// impl PreRenderer for FullBlockStatesReader {
//     fn render_xz(&self, x: u8, z: u8, chunk_y: i8, north_y: i32) -> Option<RenderResult> {
//         for (y_at_chunk, palette_idx) in self.xz_top2bottom(x, z) {
//             let paletted_block = &self.inner.palette[palette_idx as usize];
//             let base_color = BASE_COLOR_MAP.get(&paletted_block.name);

//             if let Some(base_color) = base_color {
//                 if !base_color.kind.is_none() {
//                     let real_y = i32::from(y_at_chunk) * i32::from(chunk_y);
//                     let map_color = base_color.render(real_y, north_y, paletted_block.properties.as_ref());

//                     let result = RenderResult::new(map_color, real_y);
//                     return Some(result);
//                 }
//             } else {
//                 println!("[WARN] Failed get block({})", paletted_block);
//             }
//         }

//         None
//     }
// }

// impl PreRenderer for SingleBlockStatesReader {
//     fn render_xz(&self, x: u8, z: u8, chunk_y: i8, north_y: i32) -> Option<RenderResult> {
//         let paletted_block = self.get_block();
//         let base_color = BASE_COLOR_MAP.get(&paletted_block.name);

//         if let Some(base_color) = base_color {
//             if !base_color.kind.is_none() {
//                 let real_y = 15 * i32::from(chunk_y);
//                 let map_color = base_color.render(real_y, north_y, paletted_block.properties.as_ref());

//                 let result = RenderResult::new(map_color, real_y);
//                 return Some(result);
//             }
//         } else {
//             println!("[WARN] Failed get block({})", paletted_block);
//         }

//         None
//     }
// }

// pub trait BlockColorRenderer {
//     fn render(&self, y: i32, north_y: i32, properties: Option<&Properties>) -> MapColor;
// }

// impl BlockColorRenderer for BlockColor {
//     fn render(&self, y: i32, north_y: i32, properties: Option<&Properties>) -> MapColor {

//     }
// }
