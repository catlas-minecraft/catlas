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


// pub fn read_bocks_json() {
//     let blocks = Block::from_path("./assets/blocks.json");
    
//     let blocks_count = blocks.len();
//     let box_side_length = (blocks_count as f64).sqrt().ceil() as usize;

//     let chunk_count: f64 = ((blocks_count as f64) / (MINECRAFT_CHUNK_AREA_SIZE as f64)).ceil() as f64;
//     let chunk_side_length = chunk_count.sqrt().ceil() as usize;

//     println!("フロックの数: {}", blocks_count);
//     println!("フロックの辺数: {}", box_side_length);
//     println!("必要チャンク数: {}", chunk_count);
//     println!("必要チャンクの一辺: {}", chunk_side_length);

//     let mut region = Region::from_stream(fs::File::open("./chunks/r.0.0.maple.mca").unwrap()).unwrap();

//     let chunk = match region.read_chunk(0, 0).expect("Failed read chunk") {
//         Some(data) => Chunk::from_bytes(&data).expect("Failed serialize"),
//         None => panic!("Cannot read Chunk : (x: {}, z: {})", 0, 0),
//     };

//     // let se = chunk.get_section_editor_by_section_y(-4);

//     // let se = match se {
//     //     Ok(Some(se)) => se,
//     //     Err(err) => { panic!("{:?}", err) },
//     //     _ => { panic!("cannot get section editor") }
//     // };

//     println!("{:?}", chunk);

//     // for chunk_x in 0..chunk_side_length {
//     //     let current_block_pos = chunk_x * chunk_side_length * MINECRAFT_CHUNK_AREA_SIZE;
//     //     for chunk_z in 0..chunk_side_length {
//     //         let current_block_pos: usize = (current_block_pos + chunk_z * MINECRAFT_CHUNK_AREA_SIZE).into();
//     //         println!("現在のブロック数{}", current_block_pos);

//     //         let chunk = match region.read_chunk(chunk_x, chunk_z) {
//     //             Ok(Some(data)) => {
//     //                 let chunk: Chunk = fastnbt::from_bytes(&data).unwrap();
//     //                 chunk
//     //             }
//     //             Ok(None) => panic!("Cannot read Chunk : (x: {}, z: {})", chunk_x, chunk_z),
//     //             Err(e) => panic!("{e}")
//     //         };

//     //         println!("{:?}", chunk)

//     //         // for block_x in 0..MINECRAFT_CHUNK_SIZE {
//     //         //     for block_z in 0..MINECRAFT_CHUNK_SIZE {

//     //         //     }
//     //         // }

//     //     }
//     // }
    
// }
