use catlas_colors::{BlockAttributes, BlockColor, BASE_COLOR_MAP};
use catlas_models::PalettedBlock;
use catlas_reader::{SectionReader, YPos, YPosItem};

#[derive(Debug)]
pub enum RenderedPreTile<'a> {
    Normal {
        y_pos_item: YPosItem<'a>,
        block_color: BlockColor,
    },
    Water {
        y_pos: YPos,
        deps: WaterDepsLevel,
        x: u8,
        z: u8,
    },
}

impl<'a> RenderedPreTile<'a> {
    pub fn get_y_pos(&self) -> YPos {
        match self {
            RenderedPreTile::Normal { y_pos_item, .. } => y_pos_item.y_pos,
            RenderedPreTile::Water { y_pos, .. } => *y_pos,
        }
    }
}

#[derive(Debug)]
pub enum WaterDepsLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

impl From<u8> for WaterDepsLevel {
    fn from(deps: u8) -> Self {
        if deps <= 2 {
            WaterDepsLevel::Level1
        } else if deps <= 4 {
            WaterDepsLevel::Level2
        } else if deps <= 6 {
            WaterDepsLevel::Level3
        } else if deps <= 9 {
            WaterDepsLevel::Level4
        } else {
            WaterDepsLevel::Level5
        }
    }
}

pub trait PreTileRenderer<'a> {
    fn render_pre_tile(self, x: u8, z: u8) -> Option<RenderedPreTile<'a>>;
}

impl<'a, I> PreTileRenderer<'a> for I
where
    I: Iterator<Item = &'a SectionReader> + Sized + DoubleEndedIterator,
{
    fn render_pre_tile(self, x: u8, z: u8) -> Option<RenderedPreTile<'a>> {
        let mut iter = self
            .rev()
            .filter_map(|el| el.y_direction_iter(x, z))
            .flatten();

        let (block_color, y_pos_item) = loop {
            let y_pos_item = iter.next()?;

            let paletted_block = y_pos_item.paletted_block;
            let block_color = BASE_COLOR_MAP.get(&paletted_block.name);

            let Some(block_color) = block_color else {
                // println!("[WARN] Failed get block({})", paletted_block);
                continue;
            };

            if block_color.kind.is_none() {
                continue;
            }

            break (*block_color, y_pos_item);
        };

        if is_render_as_water(&block_color, y_pos_item.paletted_block) {
            let mut deps = 1;

            for _ in 0..=8 {
                let Some(y_pos_item) = iter.next() else {
                    break;
                };
                let paletted_block = y_pos_item.paletted_block;

                let Some(block_color) = BASE_COLOR_MAP.get(&paletted_block.name) else {
                    break;
                };

                if is_render_as_water(&block_color, paletted_block) {
                    match y_pos_item.kind {
                        catlas_reader::YPosItemKind::Single => {
                            deps += 10;
                            break;
                        }
                        catlas_reader::YPosItemKind::Full => deps += 1,
                    }
                } else {
                    break;
                }
            }

            let deps_level = WaterDepsLevel::from(deps);

            Some(RenderedPreTile::Water {
                y_pos: y_pos_item.y_pos,
                deps: deps_level,
                x,
                z,
            })
        } else {
            Some(RenderedPreTile::Normal {
                y_pos_item,
                block_color,
            })
        }
    }
}

pub fn is_render_as_water(block_color: &BlockColor, paletted_block: &PalettedBlock) -> bool {
    if block_color.kind.is_water() {
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
