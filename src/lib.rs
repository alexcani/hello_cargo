//! This comments goes to the crate documentation (code that owns the lib.rs)
//! # Art
//!
//! A library for modeling artistic concepts.

/// Adds one to the number
///
/// This is the documentation of the function itself
///
///  # Examples
/// ```
/// let x = 1;
/// let y = hello_cargo::add_one(x);
/// assert_eq!(y, x+1);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Export the public API
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
