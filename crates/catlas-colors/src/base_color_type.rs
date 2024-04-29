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
        match self {
            BaseColorType::Normal(base_color) => match base_color {
                BaseColor::None => {
                    true
                },
                _ => false
            },
            _ => false
        }
    }
}
