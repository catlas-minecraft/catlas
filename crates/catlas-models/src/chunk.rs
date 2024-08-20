mod section;
mod block_state;

pub use section::Section;
pub use block_state::BlockStates;
pub use block_state::FullBlockStates;
pub use block_state::SingleBlockStates;

use fastnbt::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chunk {
    pub sections: Vec<Section>,

    #[serde(rename = "xPos")]
    pub x_pos: i32,
    #[serde(rename = "yPos")]
    pub y_pos: i32,
    #[serde(rename = "zPos")]
    pub z_pos: i32
}

impl Chunk {
    pub const Y_BOTTOM: i32 = -64;

    pub fn from_bytes<'a>(input: &'a [u8]) -> error::Result<Chunk> {
        fastnbt::from_bytes(input)
    }
}
