//! Blend types (overlap, add, add_overflow, ...)

use crate::utils::color::Color;



pub const DEFAULT_BLEND_TYPE_ALPHA: BlendType = BlendType::Overlap;
pub const DEFAULT_BLEND_TYPE_COLOR: BlendType = BlendType::Overlap;



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlendTypes {
    pub alpha: BlendType,
    pub color: BlendType,
}

impl BlendTypes {

    pub fn new() -> Self {
        BlendTypes { alpha: DEFAULT_BLEND_TYPE_ALPHA, color: DEFAULT_BLEND_TYPE_COLOR }
    }

    pub fn from(alpha: BlendType, color: BlendType) -> Self {
        BlendTypes { alpha, color }
    }

}



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlendType {
    Overlap,
    Add,
    Max,
    Min,
    AddOverflow,
    Avg,

    // TODO: add other?:
    //   - minus
    //   - multiply
}



// TODO: blend type different for alpha and pure color
pub fn blend_colors(color1: Color, color2: Color, blend_types: BlendTypes) -> Color {
    // TODO
    color2
}
