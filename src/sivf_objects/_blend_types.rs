//! Blend types (overlap, add, add_overflow, ...)

use crate::utils::color::Color;



pub enum BlendType {
    FullOverlap,
    Overlap,
    Add,
    AddOverflow,
    // TODO
}

pub fn blend(pixel1: Color, pixel2: Color, blend_type: BlendType) -> Color {
    match blend_type {
        BlendType::FullOverlap => {
            pixel2
        }
        _ => {
            todo!()
        }
    }
}
