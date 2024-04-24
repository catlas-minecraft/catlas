use serde::{Serialize, Deserialize};

use crate::models::chunk::BlockStates;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    pub block_states: Option<BlockStates>,
    #[serde(rename = "Y")]
    pub y: i8
}

impl Section {
    pub const SIZE: u8 = 16;
}
