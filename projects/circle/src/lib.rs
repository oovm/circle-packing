#![allow(non_upper_case_globals)]
#![allow(mixed_script_confusables)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../readme.md")]
// #![no_std]

mod circle;
mod ellipse;
mod extension;
mod line;
mod point;

pub use crate::float::{π, Float};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The representation of a ellipse.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
pub struct Ellipse {
    a: Float,
    b: Float,
    c: Float,
    d: Float,
    e: Float,
    f: Float,
}

/// The representation of a circle.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
pub struct Circle {
    /// Center of the circle.
    pub center: Point,
    /// Radius of the circle.
    pub radius: Float,
}

/// The representation of a line.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
pub struct Line {
    /// Start of the line.
    pub start: Point,
    /// End of the line.
    pub end: Point,
}

/// The representation of a point.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
pub struct Point {
    /// The x-coordinate.
    pub x: Float,
    /// The y-coordinate.
    pub y: Float,
}

// noinspection NonAsciiCharacters
#[cfg(feature = "f32")]
mod float {
    /// Float Type
    pub type Float = f32;

    /// constant π
    pub const π: Float = core::f32::consts::PI;
}

// noinspection NonAsciiCharacters
#[cfg(feature = "f64")]
mod float {
    /// Float Type
    pub type Float = f64;

    /// constant π
    pub const Pi: Float = core::f64::consts::PI;
}
