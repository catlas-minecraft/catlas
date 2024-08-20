use std::io::{Write, Read, Seek};

use catlas_colors::MapColor;
use catlas_models::{Chunk, Section};
use catlas_reader::ChunkReader;
use fastanvil::Region;

use crate::{RenderPreTile, RenderTile, error::{CatlasRenderError, Result}};

const REGION_SIZE: usize = 32;

pub trait Render {
    type Map;
    type NorthYCoords;

    fn render(&mut self, north_y_coords: &mut Self::NorthYCoords) -> Self::Map;
}


impl<'a: 'b, 'b> Render for ChunkReader
{
    type Map = [MapColor; Section::SIZE as usize * Section::SIZE as usize];
    type NorthYCoords = [i32; Section::SIZE as usize];

    fn render(&mut self, north_y_coords: &mut Self::NorthYCoords) -> Self::Map {
        const NONE: MapColor = MapColor::none();
        let mut map: Self::Map = [NONE; Section::SIZE as usize * Section::SIZE as usize];

        for z in 0..Section::SIZE {
            for x in 0..Section::SIZE {
                let north_y = north_y_coords[x as usize];

                let pre_rendered = self.sections.render_pre_tile(x, z);

                north_y_coords[x as usize] = match &pre_rendered {
                    Some(pre_rendered) => pre_rendered.y_pos_item.0.real_y(),
                    None => Chunk::Y_BOTTOM
                };

                let map_color = self.sections.render_tile(x, z, north_y, pre_rendered);

                map[z as usize * Section::SIZE as usize + x as usize] = map_color;
            }
        }

        map
    }
}

impl<S> Render for Region<S>
where
    S: Read + Write + Seek
{
    type Map = Result<[MapColor; Section::SIZE as usize * Section::SIZE as usize * REGION_SIZE * REGION_SIZE]>;
    type NorthYCoords = [[i32; Section::SIZE as usize]; REGION_SIZE];

    fn render(&mut self, north_y_coords: &mut Self::NorthYCoords) -> Self::Map {
        const NONE: MapColor = MapColor::none();
        let mut map = [NONE; Section::SIZE as usize * Section::SIZE as usize * REGION_SIZE * REGION_SIZE];

        for x in 0..REGION_SIZE {
            for z in 0..REGION_SIZE {
                let Some(chunk) = self.read_chunk(x, z).map_err(CatlasRenderError::ReadChunkError)? else {
                    north_y_coords[x] = [Chunk::Y_BOTTOM; Section::SIZE as usize];
                    continue;
                };

                let mut chunk: ChunkReader = fastnbt::from_bytes::<Chunk>(&chunk).map_err(CatlasRenderError::ChunkDerError)?.into();
                let chunk_map = chunk.render(&mut north_y_coords[x]);

                let start = x * Section::SIZE as usize + z * (Section::SIZE as usize).pow(2) * REGION_SIZE;

                for chunk_z in 0..Section::SIZE as usize {
                    let start = start + chunk_z * Section::SIZE as usize * REGION_SIZE;
                    let chunk_start = chunk_z * Section::SIZE as usize;

                    map[start..start + Section::SIZE as usize]
                        .clone_from_slice(
                            &chunk_map[chunk_start..chunk_start + Section::SIZE as usize]
                        );
                }
            }
        }

        Ok(map)
    }
}
