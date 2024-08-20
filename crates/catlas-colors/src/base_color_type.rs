use crate::BaseColor;

#[derive(Debug, Clone, Copy)]
pub enum BaseColorType {
    Normal(BaseColor),
    Bed(BaseColor),
    /// Axis y, other
    Axis(BaseColor, BaseColor)
}

impl BaseColorType {
    pub fn is_none(&self) -> bool {
        let BaseColorType::Normal(base_color) = self else {
            return false;
        };

        match base_color {
            BaseColor::None => {
                true
            },
            _ => false
        }
    }
}
