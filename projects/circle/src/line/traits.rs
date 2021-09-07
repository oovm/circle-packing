use super::*;

impl From<(f32, f32, f32, f32)> for Line {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        Self { start: Point::new(v.0, v.1), end: Point::new(v.2, v.3) }
    }
}
