#[derive(Debug)]
pub enum BaseColorId {
    Normal(i8),
    Bed(i8),
    // Axis y, other
    Axis(i8, i8)
}

use BaseColorId::*;

include!(concat!(env!("OUT_DIR"), "/base_color_id_map.rs"));
