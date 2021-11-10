use crate::{Float, Point};

impl Point {
    /// Construct new point
    pub fn new(x: Float, y: Float) -> Self {
        Self { x, y }
    }
    /// View point as tuple
    pub fn as_tuple(&self) -> (Float, Float) {
        (self.x, self.y)
    }
    /// Distance between two points.
    pub fn distance_to(&self, other: &Self) -> Float {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    /// TeX representation of point
    pub fn equation(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl From<(Float, Float)> for Point {
    fn from(point: (Float, Float)) -> Self {
        Self { x: point.0, y: point.1 }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
