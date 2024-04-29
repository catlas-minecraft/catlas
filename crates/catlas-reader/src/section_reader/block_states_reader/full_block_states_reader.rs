use std::cmp;

use catlas_models::{FullBlockStates, Section};

#[derive(Debug)]
pub struct FullBlockStatesReader {
    pub inner: FullBlockStates,
    block_bits: u32,
    block_per_element: u32,
    data_vec_len: u32,
    block_bit_mask: u16
}

impl FullBlockStatesReader {
    const MIN_BITS_PER_DATA: u32 = 4;

    /// 
    /// Returns the number of digits in binary for the number of pallets
    /// ```
    /// let palette_len: u8    = 0b00000111;
    /// let zeros          = palette_len.leadning_zeros(); // Return 5
    /// 
    /// assert!(u8::BITS - zeros == 3);
    /// ```
    /// 
    pub fn to_data_bits(palette_len: usize) -> u32 {
        cmp::max(
            FullBlockStatesReader::MIN_BITS_PER_DATA,
            usize::BITS - (palette_len).leading_zeros()
        )
    }

    pub fn calc_block_per_element(bits: u32) -> u32 {
        u64::BITS / bits
    }

    /// 
    /// ```
    /// assert!(0b111 == SectionReader::to_bit_mask(3));
    /// assert!(0b11111 == SectionReader::to_bit_mask(5));
    /// ```
    /// 
    pub fn to_bit_mask(data_bits: u32) -> u16 {
        (1 << data_bits) - 1
    }

    pub fn new(block_states: FullBlockStates) -> FullBlockStatesReader {
        let block_bits = FullBlockStatesReader::to_data_bits(block_states.palette.len());
        let block_per_element = FullBlockStatesReader::calc_block_per_element(block_bits);
        let data_vec_len = ((Section::SIZE as u32) * (Section::SIZE as u32) * (Section::SIZE as u32)).div_ceil(block_per_element);
        let block_bit_mask = FullBlockStatesReader::to_bit_mask(block_bits);

        if block_states.data.len() != data_vec_len as usize {
            println!("[warn] real block_data len mismatches calculated len")
        }

        FullBlockStatesReader {
            inner: block_states,
            block_bits,
            block_per_element,
            data_vec_len,
            block_bit_mask
        }
    }

    pub fn get_block_by_xyz(&self, x: u8, y: u8, z: u8) -> u16 {
        let block_pos = (y as u16) * (Section::SIZE as u16) * (Section::SIZE as u16)
            + (z as u16) * (Section::SIZE as u16)
            + (x as u16);

        let block_at_vec = (block_pos as u32) / self.block_per_element;
        let block_at_ele = (block_pos as u32) % self.block_per_element;

        let shifted = (self.inner.data[block_at_vec as usize] as u64) >> (block_at_ele * self.block_bits);

        shifted as u16 & self.block_bit_mask
    }

    pub fn topdown_iter(&self, x: u8, z: u8) -> XZTopDownSectionIter {
        XZTopDownSectionIter::new(self, x, z)
    }
}

impl From<FullBlockStates> for FullBlockStatesReader {
    fn from(value: FullBlockStates) -> Self {
        FullBlockStatesReader::new(value)
    }
}

/// Arguments x ,z y 15 ~ iterates to 0
pub struct XZTopDownSectionIter<'a> {
    x: u8,
    y: u8,
    z: u8,
    sect: &'a FullBlockStatesReader
}

impl<'a> XZTopDownSectionIter<'a> {
    pub fn new(sect: &'a FullBlockStatesReader, x: u8, z: u8) -> XZTopDownSectionIter {
        Self::with_start(&sect, x, 16, z)
    }

    pub fn with_start(sect: &'a FullBlockStatesReader, x: u8, y: u8, z: u8) -> XZTopDownSectionIter {
        XZTopDownSectionIter {
            x,
            y,
            z,
            sect
        }
    }

    pub fn current_y(&self) -> u8 {
        self.y
    }
}

/// 
/// 
/// ```
/// let section_reader = SectionReader::new();
/// 
/// for (y, pallette_idx) in section_reader.topdown_iter() {
///     // Your code
/// }
/// ```
/// 
impl<'a> Iterator for XZTopDownSectionIter<'a> {
    type Item = (u8, u16);

    fn next(&mut self) -> Option<Self::Item> {
        let item = if let Some(y) = self.y.checked_sub(1) {
            self.y = y;

            Some((y, self.sect.get_block_by_xyz(self.x, y, self.z)))
        } else {
            None
        };

        item
    }
}
