
use std::collections::HashMap;

use crate::PalettedBlock;

#[derive(Debug)]
pub struct PaletteCollector {
    data: HashMap<String, PalettedBlock>
}

impl PaletteCollector {
    pub fn new() -> PaletteCollector {
        PaletteCollector {
            data: HashMap::new()
        }
    }
}
