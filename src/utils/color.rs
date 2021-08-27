//! Color

use std::num::ParseIntError;



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorModel {
    ARGB,
    RGBA,
    // TODO LATER: add `CMYA`, `ACMY`, `XYZ`?
}

impl ColorModel {

    pub fn from(str: &str) -> Self {
        match str {
            str if str == "ARGB" => {
                ColorModel::ARGB
            }
            str if str == "RGBA" => {
                ColorModel::RGBA
            }
            _ => { panic!() }
        }
    }

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {

    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Color { a, r, g, b }
    }

    pub fn from(str: &str) -> Self {
        Color {
            a: u8::from_str_radix(&str[0..=1], 16).unwrap(),
            r: u8::from_str_radix(&str[2..=3], 16).unwrap(),
            g: u8::from_str_radix(&str[4..=5], 16).unwrap(),
            b: u8::from_str_radix(&str[6..=7], 16).unwrap(),
        }
    }

}



pub const TRANSPARENT: Color = Color { a: 0, r: 0,  g:   0, b:   0 };

pub const WHITE   : Color = Color { a: 255, r: 255, g: 255, b: 255 };
pub const BLACK   : Color = Color { a: 255, r:   0, g:   0, b:   0 };

pub const GREY_32 : Color = Color { a: 255, r:  32, g:  32, b:  32 };
pub const GREY_64 : Color = Color { a: 255, r:  64, g:  64, b:  64 };
pub const GREY_96 : Color = Color { a: 255, r:  96, g:  96, b:  96 };
pub const GREY_128: Color = Color { a: 255, r: 128, g: 128, b: 128 };
pub const GREY_160: Color = Color { a: 255, r: 160, g: 160, b: 160 };
pub const GREY_192: Color = Color { a: 255, r: 192, g: 192, b: 192 };
pub const GREY_224: Color = Color { a: 255, r: 224, g: 224, b: 224 };

pub const RED     : Color = Color { a: 255, r: 255, g:   0, b:   0 };
pub const GREEN   : Color = Color { a: 255, r:   0, g: 255, b:   0 };
pub const BLUE    : Color = Color { a: 255, r: 000, g:   0, b: 255 };

pub const CYAN    : Color = Color { a: 255, r:   0, g: 255, b: 255 };
pub const MAGENTA : Color = Color { a: 255, r: 255, g:   0, b: 255 };
pub const YELLOW  : Color = Color { a: 255, r: 255, g: 255, b:   0 };



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_ok() {
        let expected: Color = Color::new(0x11, 0x22, 0x33, 0x44);
        let actual  : Color = Color::from("11223344");
        assert_eq!(expected, actual);
    }

}
