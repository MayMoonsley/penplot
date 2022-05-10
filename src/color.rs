use std::convert::TryInto;
use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
#[derive(Copy, Clone, Debug)]
pub struct Color(pub f32, pub f32, pub f32, pub f32); // RGBA in [0, 1]

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "#{:02X?}{:02X?}{:02X?}{:02X?}",
            self.red_byte(),
            self.green_byte(),
            self.blue_byte(),
            self.alpha_byte()
        )
    }
}

impl Color {
    pub fn from_bytes(r: u8, g: u8, b: u8, a: u8) -> Color {
        let (r, g, b, a) = (r as f32, g as f32, b as f32, a as f32);
        Color(r / 255.0, g / 255.0, b / 255.0, a / 255.0)
    }

    pub fn from_ints(r: usize, g: usize, b: usize, a: usize) -> Option<Color> {
        Some(Color::from_bytes(r.try_into().ok()?, g.try_into().ok()?, b.try_into().ok()?, a.try_into().ok()?))
    }

    pub fn overlay(top: Color, bottom: Color) -> Color {
        if top.alpha() == 0.0 && bottom.alpha() == 0.0 {
            // avoid division by zero errors
            Color(0.0, 0.0, 0.0, 0.0)
        } else {
            let inv_alpha = 1.0 - top.alpha();
            let new_alpha = top.alpha() + bottom.alpha() * inv_alpha;
            let scale = 1.0 / new_alpha;
            Color(
                (top.red() * top.alpha() + bottom.red() * bottom.alpha() * inv_alpha) * scale,
                (top.green() * top.alpha() + bottom.green() * bottom.alpha() * inv_alpha) * scale,
                (top.blue() * top.alpha() + bottom.blue() * bottom.alpha() * inv_alpha) * scale,
                new_alpha,
            )
        }
    }

    #[inline]
    pub fn red(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn green(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn blue(&self) -> f32 {
        self.2
    }

    #[inline]
    pub fn alpha(&self) -> f32 {
        self.3
    }

    #[inline]
    pub fn red_byte(&self) -> u8 {
        (self.0 * 255.0) as u8
    }

    #[inline]
    pub fn green_byte(&self) -> u8 {
        (self.1 * 255.0) as u8
    }

    #[inline]
    pub fn blue_byte(&self) -> u8 {
        (self.2 * 255.0) as u8
    }

    #[inline]
    pub fn alpha_byte(&self) -> u8 {
        (self.3 * 255.0) as u8
    }
}

#[derive(Debug)]
pub enum ColorParseError {
    NoHash,
    BadInt,
    WrongLength,
}

impl Display for ColorParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ColorParseError::NoHash => write!(f, "missing #"),
            ColorParseError::BadInt => write!(f, "malformed int"),
            ColorParseError::WrongLength => write!(f, "wrong length"),
        }
    }
}

impl From<ParseIntError> for ColorParseError {
    fn from(_err: ParseIntError) -> Self {
        ColorParseError::BadInt
    }
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        if hex_code.len() != 7 && hex_code.len() != 9 {
            Err(ColorParseError::WrongLength)
        } else if &hex_code[0..1] != "#" {
            Err(ColorParseError::NoHash)
        } else {
            let r: f32 = u8::from_str_radix(&hex_code[1..3], 16)? as f32;
            let g: f32 = u8::from_str_radix(&hex_code[3..5], 16)? as f32;
            let b: f32 = u8::from_str_radix(&hex_code[5..7], 16)? as f32;
            if hex_code.len() == 9 {
                let a: f32 = u8::from_str_radix(&hex_code[7..9], 16)? as f32;
                Ok(Color(r / 255.0, g / 255.0, b / 255.0, a / 255.0))
            } else {
                Ok(Color(r / 255.0, g / 255.0, b / 255.0, 1.0))
            }
        }
    }
}
