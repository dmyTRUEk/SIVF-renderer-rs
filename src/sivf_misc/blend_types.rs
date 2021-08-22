//! Blend types (overlap, add, add_overflow, ...)

use serde_derive::{Serialize, Deserialize};

use crate::utils::color::Color;



#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlendType {
    FullOverlap,
    Overlap,
    Add,
    AddOverflow,
    // TODO
}

pub fn blend_pixels(pixel1: Color, pixel2: Color, blend_type: BlendType) -> Color {
    match blend_type {
        BlendType::FullOverlap => {
            pixel2
        }
        _ => {
            todo!()
        }
    }
}
