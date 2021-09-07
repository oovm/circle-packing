#![allow(non_upper_case_globals)]
#![allow(mixed_script_confusables)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

mod circle;
mod ellipse;
mod extension;
mod line;
mod point;

pub use crate::float::{π, Float};

/// The representation of a ellipse.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ellipse {
    a: Float,
    b: Float,
    c: Float,
    d: Float,
    e: Float,
    f: Float,
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    /// Center of the circle.
    pub center: Point,
    /// Radius of the circle.
    pub radius: Float,
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    /// Start of the line.
    pub start: Point,
    /// End of the line.
    pub end: Point,
}

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub const π: Float = std::f32::consts::PI;
}

// noinspection NonAsciiCharacters
#[cfg(feature = "f64")]
mod float {
    /// Float Type
    pub type Float = f64;
    /// constant π
    pub const Pi: Float = std::f64::consts::PI;
}