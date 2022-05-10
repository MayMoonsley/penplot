use fixed::{types::extra::U8, FixedU8};
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

impl Color {
    pub fn transparent() -> Color {
        Color(0, 0, 0, 0)
    }

    pub fn from_ints(r: usize, g: usize, b: usize, a: usize) -> Option<Color> {
        Some(Color(r.try_into().ok()?, g.try_into().ok()?, b.try_into().ok()?, a.try_into().ok()?))
    }

    fn from_fixed(r: FixedU8<U8>, g: FixedU8<U8>, b: FixedU8<U8>, a: FixedU8<U8>) -> Color {
        Color(r.to_bits(), g.to_bits(), b.to_bits(), a.to_bits())
    }

    pub fn overlay(top: Color, bottom: Color) -> Color {
        if top.alpha() == 0 && bottom.alpha() == 0 {
            // avoid division by zero errors
            Color::transparent()
        } else {
            let inv_alpha = FixedU8::<U8>::from_bits(255 - top.alpha());
            let new_alpha = top.alpha_fixed() + bottom.alpha_fixed() * inv_alpha;
            let scale = FixedU8::<U8>::from_bits(255) / new_alpha;
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
    fn red_fixed(&self) -> FixedU8<U8> {
        FixedU8::from_bits(self.0)
    }

    #[inline]
    fn green_fixed(&self) -> FixedU8<U8> {
        FixedU8::from_bits(self.1)
    }

    #[inline]
    fn blue_fixed(&self) -> FixedU8<U8> {
        FixedU8::from_bits(self.2)
    }

    #[inline]
    fn alpha_fixed(&self) -> FixedU8<U8> {
        FixedU8::from_bits(self.3)
    }
}