mod block_state;
mod section;

pub use block_state::BlockStates;
pub use block_state::FullBlockStates;
pub use block_state::SingleBlockStates;
use section::RawSection;
pub use section::Section;

use serde::de::{self};
use serde::Deserializer;
use serde::{Deserialize, Serialize};

use crate::ResolveByVersion;

#[derive(Serialize, Debug, Clone)]
pub struct Chunk {
    pub data_version: u32,
    pub sections: Vec<Section>,

    pub x_pos: i32,
    pub z_pos: i32,
}

impl Chunk {
    pub const Y_BOTTOM: i32 = -64;

    pub fn from_bytes<'a>(input: &'a [u8]) -> fastnbt::error::Result<Chunk> {
        fastnbt::from_bytes(input)
    }
}

impl<'de> Deserialize<'de> for Chunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_chunk = RawChunk::deserialize(deserializer)?;
        let data_version = raw_chunk.data_version;

        raw_chunk.resolve_by_version(data_version)
    }
}

#[derive(Deserialize)]
pub(crate) struct RawChunk {
    #[serde(rename = "DataVersion")]
    data_version: u32,
    sections: Option<Vec<RawSection>>,

    #[serde(rename = "Level")]
    level: Option<RawLevel>,

    #[serde(rename = "xPos")]
    pub x_pos: Option<i32>,
    #[serde(rename = "zPos")]
    pub z_pos: Option<i32>,
}

impl<E> ResolveByVersion<Chunk, E> for RawChunk
where
    E: serde::de::Error,
{
    fn resolve_by_version(self, data_version: u32) -> Result<Chunk, E> {
        // https://ja.minecraft.wiki/w/Java_Edition_21w43a
        if data_version >= 2844 {
            let sections = self
                .sections
                .ok_or_else(|| de::Error::missing_field("sections"))?
                .resolve_by_version(data_version)?;

            let x_pos = self.x_pos.ok_or_else(|| de::Error::missing_field("xPos"))?;
            let z_pos = self.z_pos.ok_or_else(|| de::Error::missing_field("zPos"))?;

            Ok(Chunk {
                data_version,
                sections,
                x_pos,
                z_pos,
            })
        } else {
            let level = self
                .level
                .ok_or_else(|| de::Error::missing_field("Level"))?;

            let sections = level.sections.resolve_by_version(data_version)?;
            let x_pos = level
                .x_pos
                .ok_or_else(|| de::Error::missing_field("xPos"))?;
            let z_pos = level
                .z_pos
                .ok_or_else(|| de::Error::missing_field("zPos"))?;

            Ok(Chunk {
                data_version,
                sections,
                x_pos,
                z_pos,
            })
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct RawLevel {
    #[serde(rename = "Sections")]
    sections: Vec<RawSection>,

    #[serde(rename = "xPos")]
    pub x_pos: Option<i32>,
    #[serde(rename = "zPos")]
    pub z_pos: Option<i32>,
}
