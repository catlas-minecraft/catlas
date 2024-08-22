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
