use fastnbt::LongArray;
use serde::{Serialize, Deserialize};

use crate::PalettedBlock;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BlockStates {
    FullBlockStates(FullBlockStates),
    SingleBlockStates(SingleBlockStates)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullBlockStates {
    pub palette: Vec<PalettedBlock>,
    pub data: LongArray
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleBlockStates {
    pub palette: [PalettedBlock; 1]
}
