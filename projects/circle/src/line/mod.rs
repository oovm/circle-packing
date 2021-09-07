mod traits;
use super::*;

impl Line {
    /// Create a new line from 2 points
    pub fn from_2_points(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}
