use std::fmt::Display;

use bitflags::{bitflags, parser};

bitflags! {
    pub struct BlockAttributes: u8 {
        const WATERLOGGED = 1 << 0;
        const TYPE        = 1 << 1;
        const HALF        = 1 << 2;
        const OPEN        = 1 << 3;
    }
}

impl Display for BlockAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        parser::to_writer(self, f)
    }
}
