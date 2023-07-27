//! A simple library for representing colours
//!
//! # Examples
//!
//! ```
//! use colour::Colour;
//!
//! // Make a new colour using a hex value
//! let white = Colour(0xffffff);
//! // Make a new colour using built-in constants
//! let blue = Colour::BLUE;
//! // Make a new colour using RGB values
//! let yellow = Colour::from_rgb(254, 231, 92);
//! ```
//!
//! # Feature flags
//!
//! `serde` - Enable serde features

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, ops};

/// A representation of a colour
///
/// # Examples
///
/// ```
/// use colour::Colour;
///
/// // Make a new colour using a hex value
/// let white = Colour(0xffffff);
/// assert_eq!(*white, 16777215);
///
/// // Make a new colour using built-in constants
/// let blue = Colour::BLUE;
/// assert_eq!((52, 152, 219), blue.into());
///
/// // Make a new colour using RGB values
/// let yellow = Colour::from_rgb(254, 231, 92);
/// assert_eq!(yellow, Colour::YELLOW);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Colour(pub u32);

impl Colour {
    pub const WHITE: Self = Self(0xffffff);
    pub const BLACK: Self = Self(0x000000);
    pub const AQUA: Self = Self(0x1ABC9C);
    pub const GREEN: Self = Self(0x57F287);
    pub const BLUE: Self = Self(0x3498DB);
    pub const YELLOW: Self = Self(0xFEE75C);
    pub const PURPLE: Self = Self(0x9B59B6);
    pub const GOLD: Self = Self(0xF1C40F);
    pub const ORANGE: Self = Self(0xE67E22);
    pub const RED: Self = Self(0xED4245);
    pub const GREY: Self = Self(0x95A5A6);
    pub const NAVY: Self = Self(0x34495E);
    pub const DARK_AQUA: Self = Self(0x11806A);
    pub const DARK_GREEN: Self = Self(0x1F8B4C);
    pub const DARK_BLUE: Self = Self(0x206694);
    pub const DARK_PURPLE: Self = Self(0x71368A);
    pub const DARK_GOLD: Self = Self(0xC27C0E);
    pub const DARK_ORANGE: Self = Self(0xA84300);
    pub const DARK_RED: Self = Self(0x992D22);
    pub const DARK_GREY: Self = Self(0x979C9F);
    pub const DARK_NAVY: Self = Self(0x2C3E50);
    pub const LIGHT_GREY: Self = Self(0xBCC0C0);

    /// Make a new colour using RGB values
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self((red as u32) << 16 | (green as u32) << 8 | blue as u32)
    }

    /// Get the red RGB component of the colour
    pub const fn red(self) -> u8 {
        ((self.0 >> 16) & 255) as u8
    }

    /// Get the green RGB component of the colour
    pub const fn green(self) -> u8 {
        ((self.0 >> 8) & 255) as u8
    }

    /// Get the blue RGB component of the colour
    pub const fn blue(self) -> u8 {
        (self.0 & 255) as u8
    }
}

impl ops::Add for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(*self + *rhs)
    }
}

impl ops::AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Deref for Colour {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Colour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:x}", self.0)
    }
}

impl ops::Div for Colour {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(*self / *rhs)
    }
}

impl ops::DivAssign for Colour {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl From<u32> for Colour {
    fn from(colour: u32) -> Self {
        Colour(colour)
    }
}

impl From<Colour> for u32 {
    fn from(colour: Colour) -> Self {
        *colour
    }
}

impl From<(u8, u8, u8)> for Colour {
    fn from((red, green, blue): (u8, u8, u8)) -> Self {
        Self::from_rgb(red, green, blue)
    }
}

impl From<Colour> for (u8, u8, u8) {
    fn from(colour: Colour) -> Self {
        (colour.red(), colour.green(), colour.blue())
    }
}

impl ops::Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(*self * *rhs)
    }
}

impl ops::MulAssign for Colour {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Sub for Colour {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(*self - *rhs)
    }
}

impl ops::SubAssign for Colour {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
