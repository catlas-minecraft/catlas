use std::cmp::Ordering;

use catlas_colors::{BaseColor, BaseColorType, BlockColor, MapColor, MapColorLevel};
use catlas_reader::YPosItem;

use crate::{RenderedPreTile, WaterDepsLevel};

pub fn render_tile<'a>(north_y: i32, rendered_pre_tile: Option<RenderedPreTile<'a>>) -> MapColor {
    let Some(rendered_pre_tile) = &rendered_pre_tile else {
        return MapColor::none();
    };
    match rendered_pre_tile {
        RenderedPreTile::Normal {
            y_pos_item,
            block_color,
        } => render_normal_tile(block_color, y_pos_item, north_y),
        RenderedPreTile::Water { deps, x, z, .. } => render_water_tile(deps, *x, *z),
    }
}

fn render_normal_tile(block_color: &BlockColor, y_pos_item: &YPosItem, north_y: i32) -> MapColor {
    let YPosItem {
        y_pos,
        paletted_block,
        ..
    } = &y_pos_item;
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

fn render_water_tile(deps: &WaterDepsLevel, x: u8, z: u8) -> MapColor {
    match deps {
        WaterDepsLevel::Level1 => MapColor::new(BaseColor::Water, MapColorLevel::Light),
        WaterDepsLevel::Level2 => {
            if to_checker_pattern_cell(x, z) {
                MapColor::new(BaseColor::Water, MapColorLevel::Light)
            } else {
                MapColor::new(BaseColor::Water, MapColorLevel::Normal)
            }
        }
        WaterDepsLevel::Level3 => MapColor::new(BaseColor::Water, MapColorLevel::Normal),
        WaterDepsLevel::Level4 => {
            if to_checker_pattern_cell(x, z) {
                MapColor::new(BaseColor::Water, MapColorLevel::Normal)
            } else {
                MapColor::new(BaseColor::Water, MapColorLevel::Dark)
            }
        }
        WaterDepsLevel::Level5 => MapColor::new(BaseColor::Water, MapColorLevel::Dark),
    }
}

fn to_checker_pattern_cell(x: u8, z: u8) -> bool {
    x & 1 ^ z & 1 == 0
}
