use serde::{Serialize, Deserialize};

use crate::models::chunk::BlockStates;

#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    pub block_states: Option<BlockStates>,
    #[serde(rename = "Y")]
    pub y: i8
}
