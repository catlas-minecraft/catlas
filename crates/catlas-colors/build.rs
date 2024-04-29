#[macro_use]
extern crate num_derive;

mod map_color;
mod blocks;

use core::panic;
use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path
};

use num_traits::FromPrimitive;
use regex::Regex;
use rustc_hash::FxHasher;

use map_color::{
    ColorType,
    Record, State
};

use crate::{blocks::Block, map_color::find_id_by_states};

type Hasher = BuildHasherDefault<FxHasher>;

include!("./src/block_attributes.rs");
include!("./src/base_color.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_base_color_id_map()?;

    Ok(())
}

fn build_base_color_id_map() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("base_color_id_map.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let map_colors = File::open("assets/basecolorid_map.csv").unwrap();

    let block_state_regex = Regex::new(r"^Block\{(?<block>.+)\}\[(?<properties>.+)\]$").unwrap();
    let normal_block_regex = Regex::new(r"^(?<block>.+)\[\*\]$").unwrap();

    let mut block_state_color_map: HashMap<String, ColorType, Hasher> = HashMap::default();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(map_colors);

    // Remap records
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();

        if let Some(caps) = block_state_regex.captures(&record.block) {
            let block = caps["block"].to_string();
            let properties = caps["properties"].to_string();

            match block_state_color_map.entry(block).or_insert(ColorType::Multiple(HashMap::new())) {
                ColorType::Single(_) => panic!("Expected ColorType::Single"),
                ColorType::Multiple(map) => {
                    map.insert(
                        State::try_from(&properties)?,
                        record.color_id.parse().expect("Expeceted i8")
                    );
                },
            };
        } else if let Some(caps) = normal_block_regex.captures(&record.block) {
            let block = caps["block"].to_string();

            block_state_color_map.insert(block, ColorType::Single(record.color_id.parse().expect("Expeceted i8")));
        }
    }

    // println!("result:::{:#?}", block_state_color_map);

    let blocks = Block::load("./assets/blocks.json");
    let mut map = phf_codegen::Map::<String>::new();

    for block_data in blocks {
        let block_full_name = format!("minecraft:{}", block_data.name);
        let Some(block_color) = block_state_color_map.get(&block_full_name) else {
            println!("cargo:warning=Cannot find {}", block_data.name);
            continue;
        };

        // ColorID タイプを定義
        let block_color = match block_color {
            ColorType::Single(color) => {
                let color = BaseColor::from_i8(*color).unwrap();
                format!("Normal(BaseColor::{:?})", color)
            },
            ColorType::Multiple(colors) => {
                // Bed
                if let Some(color) = find_id_by_states(colors, "part", "foot") {
                    let color = BaseColor::from_i8(*color).unwrap();
                    format!("Bed(BaseColor::{:?})", color)
                } else if let Some(axis_y_id) = find_id_by_states(colors, "axis", "y") {
                    let other_id = find_id_by_states(colors, "axis", "x").unwrap_or(axis_y_id);
                    let other_id = BaseColor::from_i8(*other_id).unwrap();
                    let axis_y_id = BaseColor::from_i8(*axis_y_id).unwrap();
                    format!("Axis(BaseColor::{:?}, BaseColor::{:?})", axis_y_id, other_id)
                } else {
                    println!("cargo:warning=Cannot find {:?}", colors);
                    String::from("Normal(0)")
                }
            }
        };

        // Attributeを定義
        let mut attributes: BlockAttributes = BlockAttributes::empty();
        for block_states in block_data.states {
            match block_states.name.as_str() {
                "waterlogged" => attributes |= BlockAttributes::WATERLOGGED,
                "type" => attributes |= BlockAttributes::TYPE,
                "half" => attributes |= BlockAttributes::HALF,
                "open" => attributes |= BlockAttributes::OPEN,
                _ => ()
            }
        }

        map.entry(block_full_name, &format!("BlockColor {{kind: {}, attr: {}}}", block_color, attributes.bits()));
    }

    write!(
        &mut file,
        "pub static BASE_COLOR_MAP: phf::Map<&'static str, BlockColor> = {}",
        map.build())
        .unwrap();

    write!(&mut file, ";\n").unwrap();

    Ok(())
}
