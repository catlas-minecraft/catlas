use crate::BaseColorType;

#[derive(Clone, Copy)]
pub struct BlockColor {
    pub kind: BaseColorType,
    pub attr: u8
}
