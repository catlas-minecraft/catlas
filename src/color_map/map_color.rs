use super::base_color::BaseColor;

#[derive(Debug, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum MapColorLevel {
    Dark,
    Normal,
    /// Default
    Light,
    Deep
}

#[derive(Debug)]
pub struct MapColor {
    pub base: BaseColor,
    pub level: MapColorLevel
}

impl Default for MapColor {
    fn default() -> Self {
        MapColor {
            base: BaseColor::None,
            level: MapColorLevel::Light
        }
    }
}

impl MapColor {
    pub fn error() -> MapColor {
        MapColor::new(BaseColor::None, MapColorLevel::Deep)
    }

    pub fn new(base_color: BaseColor, level: MapColorLevel) -> MapColor {
        MapColor {
            base: base_color,
            level
        }
    }

    pub fn rgba(&self) -> [u8; 4] {
        match self.base {
            BaseColor::None => [0, 0, 0, 0],
            BaseColor::Grass => match self.level {
                MapColorLevel::Dark => [89, 125, 39, 255],
                MapColorLevel::Normal => [109, 153, 48, 255],
                MapColorLevel::Light => [127, 178, 56, 255],
                MapColorLevel::Deep => [67, 94, 29, 255]
            },
            BaseColor::Sand => match self.level {
                MapColorLevel::Dark => [174, 164, 115, 255],
                MapColorLevel::Normal => [213, 201, 140, 255],
                MapColorLevel::Light => [247, 233, 163, 255],
                MapColorLevel::Deep => [130, 123, 86, 255]
            },
            BaseColor::Wool => match self.level {
                MapColorLevel::Dark => [140, 140, 140, 255],
                MapColorLevel::Normal => [171, 171, 171, 255],
                MapColorLevel::Light => [199, 199, 199, 255],
                MapColorLevel::Deep => [105, 105, 105, 255]
            },
            BaseColor::Fire => match self.level {
                MapColorLevel::Dark => [180, 0, 0, 255],
                MapColorLevel::Normal => [220, 0, 0, 255],
                MapColorLevel::Light => [255, 0, 0, 255],
                MapColorLevel::Deep => [135, 0, 0, 255]
            },
            BaseColor::Ice => match self.level {
                MapColorLevel::Dark => [112, 112, 180, 255],
                MapColorLevel::Normal => [138, 138, 220, 255],
                MapColorLevel::Light => [160, 160, 255, 255],
                MapColorLevel::Deep => [84, 84, 135, 255]
            },
            BaseColor::Metal => match self.level {
                MapColorLevel::Dark => [117, 117, 117, 255],
                MapColorLevel::Normal => [144, 144, 144, 255],
                MapColorLevel::Light => [167, 167, 167, 255],
                MapColorLevel::Deep => [88, 88, 88, 255]
            },
            BaseColor::Plant => match self.level {
                MapColorLevel::Dark => [0, 87, 0, 255],
                MapColorLevel::Normal => [0, 106, 0, 255],
                MapColorLevel::Light => [0, 124, 0, 255],
                MapColorLevel::Deep => [0, 65, 0, 255]
            },
            BaseColor::Snow => match self.level {
                MapColorLevel::Dark => [180, 180, 180, 255],
                MapColorLevel::Normal => [220, 220, 220, 255],
                MapColorLevel::Light => [255, 255, 255, 255],
                MapColorLevel::Deep => [135, 135, 135, 255]
            },
            BaseColor::Clay => match self.level {
                MapColorLevel::Dark => [115, 118, 129, 255],
                MapColorLevel::Normal => [141, 144, 158, 255],
                MapColorLevel::Light => [164, 168, 184, 255],
                MapColorLevel::Deep => [86, 88, 97, 255]
            },
            BaseColor::Dirt => match self.level {
                MapColorLevel::Dark => [106, 76, 54, 255],
                MapColorLevel::Normal => [130, 94, 66, 255],
                MapColorLevel::Light => [151, 109, 77, 255],
                MapColorLevel::Deep => [79, 57, 40, 255]
            },
            BaseColor::Stone => match self.level {
                MapColorLevel::Dark => [79, 79, 79, 255],
                MapColorLevel::Normal => [96, 96, 96, 255],
                MapColorLevel::Light => [112, 112, 112, 255],
                MapColorLevel::Deep => [59, 59, 59, 255]
            },
            BaseColor::Water => match self.level {
                MapColorLevel::Dark => [45, 45, 180, 255],
                MapColorLevel::Normal => [55, 55, 220, 255],
                MapColorLevel::Light => [64, 64, 255, 255],
                MapColorLevel::Deep => [33, 33, 135, 255]
            },
            BaseColor::Wood => match self.level {
                MapColorLevel::Dark => [100, 84, 50, 255],
                MapColorLevel::Normal => [123, 102, 62, 255],
                MapColorLevel::Light => [143, 119, 72, 255],
                MapColorLevel::Deep => [75, 63, 38, 255]
            },
            BaseColor::Quartz => match self.level {
                MapColorLevel::Dark => [180, 177, 172, 255],
                MapColorLevel::Normal => [220, 217, 211, 255],
                MapColorLevel::Light => [255, 252, 245, 255],
                MapColorLevel::Deep => [135, 133, 129, 255]
            },
            BaseColor::ColorOrange => match self.level {
                MapColorLevel::Dark => [152, 89, 36, 255],
                MapColorLevel::Normal => [186, 109, 44, 255],
                MapColorLevel::Light => [216, 127, 51, 255],
                MapColorLevel::Deep => [114, 67, 27, 255]
            },
            BaseColor::ColorMagenta => match self.level {
                MapColorLevel::Dark => [125, 53, 152, 255],
                MapColorLevel::Normal => [153, 65, 186, 255],
                MapColorLevel::Light => [178, 76, 216, 255],
                MapColorLevel::Deep => [94, 40, 114, 255]
            },
            BaseColor::ColorLightBlue => match self.level {
                MapColorLevel::Dark => [72, 108, 152, 255],
                MapColorLevel::Normal => [88, 132, 186, 255],
                MapColorLevel::Light => [102, 153, 216, 255],
                MapColorLevel::Deep => [54, 81, 114, 255]
            },
            BaseColor::ColorYellow => match self.level {
                MapColorLevel::Dark => [161, 161, 36, 255],
                MapColorLevel::Normal => [197, 197, 44, 255],
                MapColorLevel::Light => [229, 229, 51, 255],
                MapColorLevel::Deep => [121, 121, 27, 255]
            },
            BaseColor::ColorLightGreen => match self.level {
                MapColorLevel::Dark => [89, 144, 17, 255],
                MapColorLevel::Normal => [109, 176, 21, 255],
                MapColorLevel::Light => [127, 204, 25, 255],
                MapColorLevel::Deep => [67, 108, 13, 255]
            },
            BaseColor::ColorPink => match self.level {
                MapColorLevel::Dark => [170, 89, 116, 255],
                MapColorLevel::Normal => [208, 109, 142, 255],
                MapColorLevel::Light => [242, 127, 165, 255],
                MapColorLevel::Deep => [128, 67, 87, 255]
            },
            BaseColor::ColorGray => match self.level {
                MapColorLevel::Dark => [53, 53, 53, 255],
                MapColorLevel::Normal => [65, 65, 65, 255],
                MapColorLevel::Light => [76, 76, 76, 255],
                MapColorLevel::Deep => [40, 40, 40, 255]
            },
            BaseColor::ColorLightGray => match self.level {
                MapColorLevel::Dark => [108, 108, 108, 255],
                MapColorLevel::Normal => [132, 132, 132, 255],
                MapColorLevel::Light => [153, 153, 153, 255],
                MapColorLevel::Deep => [81, 81, 81, 255]
            },
            BaseColor::ColorCyan => match self.level {
                MapColorLevel::Dark => [53, 89, 108, 255],
                MapColorLevel::Normal => [65, 109, 132, 255],
                MapColorLevel::Light => [76, 127, 153, 255],
                MapColorLevel::Deep => [40, 67, 81, 255]
            },
            BaseColor::ColorPurple => match self.level {
                MapColorLevel::Dark => [89, 44, 125, 255],
                MapColorLevel::Normal => [109, 54, 153, 255],
                MapColorLevel::Light => [127, 63, 178, 255],
                MapColorLevel::Deep => [67, 33, 94, 255]
            },
            BaseColor::ColorBlue => match self.level {
                MapColorLevel::Dark => [36, 53, 125, 255],
                MapColorLevel::Normal => [44, 65, 153, 255],
                MapColorLevel::Light => [51, 76, 178, 255],
                MapColorLevel::Deep => [27, 40, 94, 255]
            },
            BaseColor::ColorBrown => match self.level {
                MapColorLevel::Dark => [72, 53, 36, 255],
                MapColorLevel::Normal => [88, 65, 44, 255],
                MapColorLevel::Light => [102, 76, 51, 255],
                MapColorLevel::Deep => [54, 40, 27, 255]
            },
            BaseColor::ColorGreen => match self.level {
                MapColorLevel::Dark => [72, 89, 36, 255],
                MapColorLevel::Normal => [88, 109, 44, 255],
                MapColorLevel::Light => [102, 127, 51, 255],
                MapColorLevel::Deep => [54, 67, 27, 255]
            },
            BaseColor::ColorRed => match self.level {
                MapColorLevel::Dark => [108, 36, 36, 255],
                MapColorLevel::Normal => [132, 44, 44, 255],
                MapColorLevel::Light => [153, 51, 51, 255],
                MapColorLevel::Deep => [81, 27, 27, 255]
            },
            BaseColor::ColorBlack => match self.level {
                MapColorLevel::Dark => [17, 17, 17, 255],
                MapColorLevel::Normal => [21, 21, 21, 255],
                MapColorLevel::Light => [25, 25, 25, 255],
                MapColorLevel::Deep => [13, 13, 13, 255]
            },
            BaseColor::Gold => match self.level {
                MapColorLevel::Dark => [176, 168, 54, 255],
                MapColorLevel::Normal => [215, 205, 66, 255],
                MapColorLevel::Light => [250, 238, 77, 255],
                MapColorLevel::Deep => [132, 126, 40, 255]
            },
            BaseColor::Diamond => match self.level {
                MapColorLevel::Dark => [64, 154, 150, 255],
                MapColorLevel::Normal => [79, 188, 183, 255],
                MapColorLevel::Light => [92, 219, 213, 255],
                MapColorLevel::Deep => [48, 115, 112, 255]
            },
            BaseColor::Lapis => match self.level {
                MapColorLevel::Dark => [52, 90, 180, 255],
                MapColorLevel::Normal => [63, 110, 220, 255],
                MapColorLevel::Light => [74, 128, 255, 255],
                MapColorLevel::Deep => [39, 67, 135, 255]
            },
            BaseColor::Emerald => match self.level {
                MapColorLevel::Dark => [0, 153, 40, 255],
                MapColorLevel::Normal => [0, 187, 50, 255],
                MapColorLevel::Light => [0, 217, 58, 255],
                MapColorLevel::Deep => [0, 114, 30, 255]
            },
            BaseColor::Podzol => match self.level {
                MapColorLevel::Dark => [91, 60, 34, 255],
                MapColorLevel::Normal => [111, 74, 42, 255],
                MapColorLevel::Light => [129, 86, 49, 255],
                MapColorLevel::Deep => [68, 45, 25, 255]
            },
            BaseColor::Nether => match self.level {
                MapColorLevel::Dark => [79, 1, 0, 255],
                MapColorLevel::Normal => [96, 1, 0, 255],
                MapColorLevel::Light => [112, 2, 0, 255],
                MapColorLevel::Deep => [59, 1, 0, 255]
            },
            BaseColor::TerracottaWhite => match self.level {
                MapColorLevel::Dark => [147, 124, 113, 255],
                MapColorLevel::Normal => [180, 152, 138, 255],
                MapColorLevel::Light => [209, 177, 161, 255],
                MapColorLevel::Deep => [110, 93, 85, 255]
            },
            BaseColor::TerracottaOrange => match self.level {
                MapColorLevel::Dark => [112, 57, 25, 255],
                MapColorLevel::Normal => [137, 70, 31, 255],
                MapColorLevel::Light => [159, 82, 36, 255],
                MapColorLevel::Deep => [84, 43, 19, 255]
            },
            BaseColor::TerracottaMagenta => match self.level {
                MapColorLevel::Dark => [105, 61, 76, 255],
                MapColorLevel::Normal => [128, 75, 93, 255],
                MapColorLevel::Light => [149, 87, 108, 255],
                MapColorLevel::Deep => [78, 46, 57, 255]
            },
            BaseColor::TerracottaLightBlue => match self.level {
                MapColorLevel::Dark => [79, 76, 97, 255],
                MapColorLevel::Normal => [96, 93, 119, 255],
                MapColorLevel::Light => [112, 108, 138, 255],
                MapColorLevel::Deep => [59, 57, 73, 255]
            },
            BaseColor::TerracottaYellow => match self.level {
                MapColorLevel::Dark => [131, 93, 25, 255],
                MapColorLevel::Normal => [160, 114, 31, 255],
                MapColorLevel::Light => [186, 133, 36, 255],
                MapColorLevel::Deep => [98, 70, 19, 255]
            },
            BaseColor::TerracottaLightGreen => match self.level {
                MapColorLevel::Dark => [72, 82, 37, 255],
                MapColorLevel::Normal => [88, 100, 45, 255],
                MapColorLevel::Light => [103, 117, 53, 255],
                MapColorLevel::Deep => [54, 61, 28, 255]
            },
            BaseColor::TerracottaPink => match self.level {
                MapColorLevel::Dark => [112, 54, 55, 255],
                MapColorLevel::Normal => [138, 66, 67, 255],
                MapColorLevel::Light => [160, 77, 78, 255],
                MapColorLevel::Deep => [84, 40, 41, 255]
            },
            BaseColor::TerracottaGray => match self.level {
                MapColorLevel::Dark => [40, 28, 24, 255],
                MapColorLevel::Normal => [49, 35, 30, 255],
                MapColorLevel::Light => [57, 41, 35, 255],
                MapColorLevel::Deep => [30, 21, 18, 255]
            },
            BaseColor::TerracottaLightGray => match self.level {
                MapColorLevel::Dark => [95, 75, 69, 255],
                MapColorLevel::Normal => [116, 92, 84, 255],
                MapColorLevel::Light => [135, 107, 98, 255],
                MapColorLevel::Deep => [71, 56, 51, 255]
            },
            BaseColor::TerracottaCyan => match self.level {
                MapColorLevel::Dark => [61, 64, 64, 255],
                MapColorLevel::Normal => [75, 79, 79, 255],
                MapColorLevel::Light => [87, 92, 92, 255],
                MapColorLevel::Deep => [46, 48, 48, 255]
            },
            BaseColor::TerracottaPurple => match self.level {
                MapColorLevel::Dark => [86, 51, 62, 255],
                MapColorLevel::Normal => [105, 62, 75, 255],
                MapColorLevel::Light => [122, 73, 88, 255],
                MapColorLevel::Deep => [64, 38, 46, 255]
            },
            BaseColor::TerracottaBlue => match self.level {
                MapColorLevel::Dark => [53, 43, 64, 255],
                MapColorLevel::Normal => [65, 53, 79, 255],
                MapColorLevel::Light => [76, 62, 92, 255],
                MapColorLevel::Deep => [40, 32, 48, 255]
            },
            BaseColor::TerracottaBrown => match self.level {
                MapColorLevel::Dark => [53, 35, 24, 255],
                MapColorLevel::Normal => [65, 43, 30, 255],
                MapColorLevel::Light => [76, 50, 35, 255],
                MapColorLevel::Deep => [40, 26, 18, 255]
            },
            BaseColor::TerracottaGreen => match self.level {
                MapColorLevel::Dark => [53, 57, 29, 255],
                MapColorLevel::Normal => [65, 70, 36, 255],
                MapColorLevel::Light => [76, 82, 42, 255],
                MapColorLevel::Deep => [40, 43, 22, 255]
            },
            BaseColor::TerracottaRed => match self.level {
                MapColorLevel::Dark => [100, 42, 32, 255],
                MapColorLevel::Normal => [122, 51, 39, 255],
                MapColorLevel::Light => [142, 60, 46, 255],
                MapColorLevel::Deep => [75, 31, 24, 255]
            },
            BaseColor::TerracottaBlack => match self.level {
                MapColorLevel::Dark => [26, 15, 11, 255],
                MapColorLevel::Normal => [31, 18, 13, 255],
                MapColorLevel::Light => [37, 22, 16, 255],
                MapColorLevel::Deep => [19, 11, 8, 255]
            },
            BaseColor::CrimsonNylium => match self.level {
                MapColorLevel::Dark => [133, 33, 34, 255],
                MapColorLevel::Normal => [163, 41, 42, 255],
                MapColorLevel::Light => [189, 48, 49, 255],
                MapColorLevel::Deep => [100, 25, 25, 255]
            },
            BaseColor::CrimsonStem => match self.level {
                MapColorLevel::Dark => [104, 44, 68, 255],
                MapColorLevel::Normal => [127, 54, 83, 255],
                MapColorLevel::Light => [148, 63, 97, 255],
                MapColorLevel::Deep => [78, 33, 51, 255]
            },
            BaseColor::CrimsonHyphae => match self.level {
                MapColorLevel::Dark => [64, 17, 20, 255],
                MapColorLevel::Normal => [79, 21, 25, 255],
                MapColorLevel::Light => [92, 25, 29, 255],
                MapColorLevel::Deep => [48, 13, 15, 255]
            },
            BaseColor::WarpedNylium => match self.level {
                MapColorLevel::Dark => [15, 88, 94, 255],
                MapColorLevel::Normal => [18, 108, 115, 255],
                MapColorLevel::Light => [22, 126, 134, 255],
                MapColorLevel::Deep => [11, 66, 70, 255]
            },
            BaseColor::WarpedStem => match self.level {
                MapColorLevel::Dark => [40, 100, 98, 255],
                MapColorLevel::Normal => [50, 122, 120, 255],
                MapColorLevel::Light => [58, 142, 140, 255],
                MapColorLevel::Deep => [30, 75, 74, 255]
            },
            BaseColor::WarpedHyphae => match self.level {
                MapColorLevel::Dark => [60, 31, 43, 255],
                MapColorLevel::Normal => [74, 37, 53, 255],
                MapColorLevel::Light => [86, 44, 62, 255],
                MapColorLevel::Deep => [45, 23, 32, 255]
            },
            BaseColor::WarpedWartBlock => match self.level {
                MapColorLevel::Dark => [14, 127, 93, 255],
                MapColorLevel::Normal => [17, 155, 114, 255],
                MapColorLevel::Light => [20, 180, 133, 255],
                MapColorLevel::Deep => [10, 95, 70, 255]
            },
            BaseColor::Deepslate => match self.level {
                MapColorLevel::Dark => [70, 70, 70, 255],
                MapColorLevel::Normal => [86, 86, 86, 255],
                MapColorLevel::Light => [100, 100, 100, 255],
                MapColorLevel::Deep => [52, 52, 52, 255]
            },
            BaseColor::RawIron => match self.level {
                MapColorLevel::Dark => [152, 123, 103, 255],
                MapColorLevel::Normal => [186, 150, 126, 255],
                MapColorLevel::Light => [216, 175, 147, 255],
                MapColorLevel::Deep => [114, 92, 77, 255]
            },
            BaseColor::GlowLichen => match self.level {
                MapColorLevel::Dark => [89, 117, 105, 255],
                MapColorLevel::Normal => [109, 144, 129, 255],
                MapColorLevel::Light => [127, 167, 150, 255],
                MapColorLevel::Deep => [67, 88, 79, 255]
            }
        }
    }
}

impl From<(BaseColor, MapColorLevel)> for MapColor {
    fn from((base, level): (BaseColor, MapColorLevel)) -> Self {
        MapColor::new(base, level)
    }
}
