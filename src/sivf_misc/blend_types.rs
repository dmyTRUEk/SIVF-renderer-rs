//! Blend types (overlap, add, add_overflow, ...)

use serde_derive::{Serialize, Deserialize};

use crate::utils::color::Color;



pub const DEFAULT_BLEND_TYPE_ALPHA: BlendType = BlendType::Overlap;
pub const DEFAULT_BLEND_TYPE_COLOR: BlendType = BlendType::Overlap;



#[derive(Clone, Copy, Debug, PartialEq, /*Serialize,*/ Deserialize)]
pub struct BlendTypes {
    pub alpha: BlendType,
    pub color: BlendType,
}

impl BlendTypes {

    pub fn new() -> Self {
        BlendTypes {
            alpha: DEFAULT_BLEND_TYPE_ALPHA,
            color: DEFAULT_BLEND_TYPE_COLOR,
        }
    }

}



#[derive(Clone, Copy, Debug, PartialEq, /*Serialize,*/ Deserialize)]
pub enum BlendType {

    #[serde(rename="full_overlap")]
    FullOverlap,

    #[serde(rename="overlap")]
    Overlap,

    #[serde(rename="add")]
    Add,

    #[serde(rename="add_overflow")]
    AddOverflow,

    // TODO: add other?:
    //   - minus
    //   - multiply
}



// TODO: blend type different for alpha and pure color
pub fn blend_colors(color1: Color, color2: Color, blend_types: BlendTypes) -> Color {
    // TODO
    color2
}
