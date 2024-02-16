mod error;
mod region_manager;
mod chunk_manager;
mod section_manager;

use std::fmt::Display;

pub use error::ManagerError;
pub use error::Result;

pub use region_manager::RegionManager;
pub use chunk_manager::ChunkManager;
pub use section_manager::SectionManager;

pub struct XZCoord {
    x: i32,
    z: i32
}

impl XZCoord {
    fn new(x: i32, z: i32) -> Self{
        XZCoord { x, z }
    }
}

impl From<(i32, i32)> for XZCoord {
    fn from(value: (i32, i32)) -> Self {
        XZCoord {
            x: value.0,
            z: value.1
        }
    }
}

impl Display for XZCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.z)
    }
}
