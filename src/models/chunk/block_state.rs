use fastnbt::LongArray;
use serde::{Serialize, Deserialize};

use crate::models::PalettedBlock;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BlockStates {
    FullBlockStates(FullBlockStates),
    SingleBlockStates(SingleBlockStates)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullBlockStates {
    pub palette: Vec<PalettedBlock>,
    pub data: LongArray
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleBlockStates {
    pub palette: [PalettedBlock; 1]
}
