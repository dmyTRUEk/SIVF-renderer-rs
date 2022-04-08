//! Blend types (overlap, add, add_overflow, ...)

use crate::utils::color::Color;



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlendType {
    Overlap,
    Add,
    Max,
    Min,
    // AddOverflow,
    Avg,

    // TODO: add other?:
    //   - minus
    //   - multiply
}



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlendTypes {
    pub alpha: BlendType,
    pub color: BlendType,
}

impl BlendTypes {

    pub fn overlap() -> Self {
        BlendTypes { alpha: BlendType::Overlap, color: BlendType::Overlap }
    }

    pub fn from(alpha: BlendType, color: BlendType) -> Self {
        BlendTypes { alpha, color }
    }

}



// TODO: blend type different for alpha and pure color
pub fn blend_colors(color1: &Color, color2: &Color, blend_types: &BlendTypes) -> Color {
    let a: u8 = match blend_types.alpha {
        BlendType::Overlap => {
            if color2.a != 0x00 { color2.a } else { color1.a }
        }
        _ => { todo!() }
    };
    let (r, g, b): (u8, u8, u8) = match blend_types.color {
        BlendType::Overlap => {
            if color2.a != 0x00 { (color2.r, color2.g, color2.b) } else { (color1.r, color1.g, color1.b) }
        }
        _ => { todo!() }
    };
    Color::new(a, r, g, b)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_colors_overlap_overlap() {
        let blend_types: BlendTypes = BlendTypes::overlap();
        {
            let color1: Color = Color::from("ff112233");
            let color2: Color = Color::from("ff112233");
            let expected: Color = Color::from("ff112233");
            let actual  : Color = blend_colors(&color1, &color2, &blend_types);
            assert_eq!(expected, actual);
        }
        {
            let color1: Color = Color::from("ff112233");
            let color2: Color = Color::from("ff445566");
            let expected: Color = Color::from("ff445566");
            let actual  : Color = blend_colors(&color1, &color2, &blend_types);
            assert_eq!(expected, actual);
        }
        {
            let color1: Color = Color::from("00112233");
            let color2: Color = Color::from("ff445566");
            let expected: Color = Color::from("ff445566");
            let actual  : Color = blend_colors(&color1, &color2, &blend_types);
            assert_eq!(expected, actual);
        }
        {
            let color1: Color = Color::from("ff112233");
            let color2: Color = Color::from("00445566");
            let expected: Color = Color::from("ff112233");
            let actual  : Color = blend_colors(&color1, &color2, &blend_types);
            assert_eq!(expected, actual);
        }
    }
}

