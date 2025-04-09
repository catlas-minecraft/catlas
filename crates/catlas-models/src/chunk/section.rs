use fastnbt::LongArray;
use serde::{Deserialize, Serialize};

use crate::{
    chunk::BlockStates, FullBlockStates, PalettedBlock, ResolveByVersion, SingleBlockStates,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    pub block_states: Option<BlockStates>,
    #[serde(rename = "Y")]
    pub y: i8,
}

impl Section {
    pub const SIZE: u8 = 16;
}

#[derive(Deserialize)]
pub(crate) struct RawSection {
    #[serde(rename = "BlockStates")]
    block_states_data: Option<LongArray>,
    #[serde(rename = "Palette")]
    palette: Option<Vec<PalettedBlock>>,

    block_states: Option<BlockStates>,

    #[serde(rename = "Y")]
    pub y: i8,
}

impl<E> ResolveByVersion<Section, E> for RawSection
where
    E: serde::de::Error,
{
    fn resolve_by_version(self, data_version: u32) -> Result<Section, E> {
        // https://ja.minecraft.wiki/w/Java_Edition_21w39a
        if data_version >= 2836 {
            let block_states = self.block_states;
            let y = self.y;

            Ok(Section { block_states, y })
        } else {
            let (Some(block_states_data), Some(palette)) = (self.block_states_data, self.palette)
            else {
                return Ok(Section {
                    block_states: None,
                    y: self.y,
                });
            };

            let section = if palette.len() == 1 {
                let palette_item = palette.into_iter().next().unwrap();
                let block_states = BlockStates::SingleBlockStates(SingleBlockStates {
                    palette: [palette_item],
                });

                Section {
                    block_states: Some(block_states),
                    y: self.y,
                }
            } else {
                let block_states = BlockStates::FullBlockStates(FullBlockStates {
                    data: block_states_data,
                    palette,
                });

                Section {
                    block_states: Some(block_states),
                    y: self.y,
                }
            };

            Ok(section)
        }
    }
}
