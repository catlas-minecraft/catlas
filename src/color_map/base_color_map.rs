use BaseColorType::*;

use super::{base_color::BaseColor};

#[derive(Debug, Clone, Copy)]
pub enum BaseColorType {
    Normal(BaseColor),
    Bed(BaseColor),
    /// Axis y, other
    Axis(BaseColor, BaseColor)
}

impl BaseColorType {
    pub fn is_none(&self) -> bool {
        match self {
            Normal(base_color) => match base_color {
                BaseColor::None => {
                    true
                },
                _ => false
            },
            _ => false
        }
    }
}

#[derive(Clone, Copy)]
pub struct BlockColor {
    pub kind: BaseColorType,
    pub attr: u8
}

include!(concat!(env!("OUT_DIR"), "/base_color_id_map.rs"));
