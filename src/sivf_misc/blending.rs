//! Blend types (overlap, add, add_overflow, ...)

use crate::sivf_misc::color::Color;



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
        BlendType::Add => {
            (color1.a as u16 + color2.a as u16).min(255) as u8
        }
        BlendType::Max => {
            color1.a.max(color2.a)
        }
        BlendType::Min => {
            color1.a.min(color2.a)
        }
        BlendType::Avg => { unimplemented!() }
    };
    let (r, g, b): (u8, u8, u8) = match blend_types.color {
        BlendType::Overlap => {
            if color2.a != 0x00 {
                (color2.r, color2.g, color2.b)
            } else {
                (color1.r, color1.g, color1.b)
            }
        }
        BlendType::Add => {
            (
                (color1.r as u16 + color2.r as u16).min(255) as u8,
                (color1.g as u16 + color2.g as u16).min(255) as u8,
                (color1.b as u16 + color2.b as u16).min(255) as u8
            )
        }
        BlendType::Max => {
            (
                color1.r.max(color2.r),
                color1.g.max(color2.g),
                color1.b.max(color2.b)
            )
        }
        BlendType::Min => {
            (
                color1.r.min(color2.r),
                color1.g.min(color2.g),
                color1.b.min(color2.b)
            )
        }
        BlendType::Avg => { unimplemented!() }
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

