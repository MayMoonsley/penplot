use fixed::{types::extra::U8, FixedU16};
use std::convert::TryInto;
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8, pub u8); // RGBA in [0, 255]

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Color(r, g, b, a) = self;
        write!(f, "#{:02X?}{:02X?}{:02X?}{:02X?}", r, g, b, a)
    }
}

#[inline]
fn fixed_to_byte(x: FixedU16<U8>) -> u8 {
    if x > 255 {
        255
    } else {
        x.to_bits() as u8
    }
}

impl Color {
    pub fn transparent() -> Color {
        Color(0, 0, 0, 0)
    }

    pub fn from_ints(r: usize, g: usize, b: usize, a: usize) -> Option<Color> {
        Some(Color(r.try_into().ok()?, g.try_into().ok()?, b.try_into().ok()?, a.try_into().ok()?))
    }

    #[inline]
    fn from_fixed(r: FixedU16<U8>, g: FixedU16<U8>, b: FixedU16<U8>, a: FixedU16<U8>) -> Color {
        Color(fixed_to_byte(r), fixed_to_byte(g), fixed_to_byte(b), fixed_to_byte(a))
    }

    pub fn overlay(top: Color, bottom: Color) -> Color {
        if top.alpha() == 0 && bottom.alpha() == 0 {
            // avoid division by zero errors
            Color::transparent()
        } else {
            let inv_alpha = FixedU16::<U8>::from_bits((255 - top.alpha()).into());
            let new_alpha = top.alpha_fixed() + bottom.alpha_fixed() * inv_alpha;
            let scale = FixedU16::<U8>::from_bits(255) / new_alpha;
            Color::from_fixed(
                (top.red_fixed() * top.alpha_fixed() + bottom.red_fixed() * bottom.alpha_fixed() * inv_alpha) * scale,
                (top.green_fixed() * top.alpha_fixed() + bottom.green_fixed() * bottom.alpha_fixed() * inv_alpha) * scale,
                (top.blue_fixed() * top.alpha_fixed() + bottom.blue_fixed() * bottom.alpha_fixed() * inv_alpha) * scale,
                new_alpha,
            )
        }
    }

    #[inline]
    pub fn red(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn green(&self) -> u8 {
        self.1
    }

    #[inline]
    pub fn blue(&self) -> u8 {
        self.2
    }

    #[inline]
    pub fn alpha(&self) -> u8 {
        self.3
    }

    #[inline]
    fn red_fixed(&self) -> FixedU16<U8> {
        FixedU16::from_bits(self.red().into())
    }

    #[inline]
    fn green_fixed(&self) -> FixedU16<U8> {
        FixedU16::from_bits(self.green().into())
    }

    #[inline]
    fn blue_fixed(&self) -> FixedU16<U8> {
        FixedU16::from_bits(self.blue().into())
    }

    #[inline]
    fn alpha_fixed(&self) -> FixedU16<U8> {
        FixedU16::from_bits(self.alpha().into())
    }
}