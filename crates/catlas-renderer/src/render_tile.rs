use std::cmp::Ordering;

use catlas_colors::{BaseColor, BaseColorType, BlockAttributes, MapColor, MapColorLevel};
use catlas_reader::SectionReader;

use crate::RenderPreTileResult;

pub trait RenderTile<'a> {
    fn render_tile(
        self,
        x: u8,
        z: u8,
        north_y: i32,
        render_pre_tile_result: Option<RenderPreTileResult<'a>>,
    ) -> MapColor;
}

impl<'a, II, I> RenderTile<'a> for II
where
    II: IntoIterator<Item = &'a SectionReader, IntoIter = I>,
    I: Iterator<Item = &'a SectionReader> + Sized + DoubleEndedIterator,
{
    fn render_tile(
        self,
        x: u8,
        z: u8,
        north_y: i32,
        render_pre_tile_result: Option<RenderPreTileResult<'a>>,
    ) -> MapColor {
        let Some(render_pre_tile_result) = &render_pre_tile_result else {
            return MapColor::none();
        };

        let block_color = render_pre_tile_result.block_color;
        let (y_pos, paletted_block) = &render_pre_tile_result.y_pos_item;
        let y = y_pos.real_y();

        let base_color: Option<BaseColor> = 'base: {
            match block_color.kind {
                BaseColorType::Normal(base_color) => Some(base_color),
                BaseColorType::Bed(base_color) => {
                    let Some(ref properties) = paletted_block.properties else {
                        break 'base None;
                    };
                    let Some(part) = properties.get("part") else {
                        break 'base None;
                    };

                    match part.as_str() {
                        "head" => Some(BaseColor::Wool),
                        "foot" => Some(base_color),
                        _ => None,
                    }
                }
                BaseColorType::Axis(y, other) => {
                    let Some(ref properties) = paletted_block.properties else {
                        break 'base None;
                    };
                    let Some(axis) = properties.get("axis") else {
                        break 'base None;
                    };

                    match axis.as_str() {
                        "y" => Some(y),
                        "x" | "z" => Some(other),
                        _ => None,
                    }
                }
            }
        };

        let map_color = match base_color {
            Some(base_color) => match y.cmp(&north_y) {
                Ordering::Equal => MapColor::new(base_color, MapColorLevel::Normal),
                Ordering::Greater => MapColor::new(base_color, MapColorLevel::Light),
                Ordering::Less => MapColor::new(base_color, MapColorLevel::Dark),
            },
            None => MapColor::error(),
        };

        map_color
    }
}

fn is_render_as_water<'a>(pre_tile: &RenderPreTileResult<'a>) -> bool {
    let block_color = pre_tile.block_color;
    let (_, paletted_block) = pre_tile.y_pos_item;

    if paletted_block.name == "water" {
        return true;
    }

    let attrs = BlockAttributes::from_bits_retain(block_color.attr);

    if !attrs.contains(BlockAttributes::WATERLOGGED) {
        return false;
    }

    let Some(ref properties) = paletted_block.properties else {
        return false;
    };

    let Some(waterlogged) = properties.get("waterlogged") else {
        return false;
    };

    if waterlogged != "true" {
        return false;
    }

    if attrs.contains(BlockAttributes::TYPE) {
        let Some(r#type) = properties.get("type") else {
            return false;
        };

        return r#type == "bottom";
    } else if attrs.contains(BlockAttributes::HALF) {
        let Some(half) = properties.get("half") else {
            return false;
        };

        if half == "bottom" {
            return true;
        }

        if attrs.contains(BlockAttributes::OPEN) {
            let Some(open) = properties.get("open") else {
                return false;
            };

            return open == "true";
        }

        return true;
    } else {
        return true;
    }
}
