use crate::BaseColorType;

#[derive(Clone, Copy, Debug)]
pub struct BlockColor {
    pub kind: BaseColorType,
    pub attr: u8
}
