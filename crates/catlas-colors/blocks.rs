use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use serde_json;

const MINECRAFT_CHUNK_SIZE: usize = 16;
const MINECRAFT_CHUNK_AREA_SIZE: usize = MINECRAFT_CHUNK_SIZE.pow(2);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: u32,
    pub name: String,
    pub display_name: String,
    pub states: Vec<State>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub name: String,
    #[serde(rename="type")]
    pub r#type: String
}

impl Block {
    pub fn load<P: AsRef<Path>>(path: P) -> Vec<Block> {
        let blocks = fs::read_to_string(path).expect("Failed read");
        serde_json::from_str(&blocks).expect("Failed read")
    }
}
