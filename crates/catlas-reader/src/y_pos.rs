use catlas_models::PalettedBlock;

#[derive(Debug, Clone, Copy)]
pub struct YPos {
    section_y: i8,
    y_in_section: u8,
}

impl YPos {
    pub fn new(section_y: i8, y_in_section: u8) -> YPos {
        YPos {
            section_y,
            y_in_section,
        }
    }

    pub fn real_y(&self) -> i32 {
        i32::from(self.section_y) * 16 + i32::from(self.y_in_section)
    }
}

#[derive(Debug)]
pub struct YPosItem<'a> {
    pub y_pos: YPos,
    pub paletted_block: &'a PalettedBlock,
    pub kind: YPosItemKind,
}

#[derive(Debug)]
pub enum YPosItemKind {
    Single,
    Full,
}
