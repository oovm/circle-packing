mod traits;
use super::*;

impl Line {
    pub fn from_2_points(start: &Point, end: &Point) -> Self {
        Self { start: *start, end: *end, ..Default::default() }
    }
}

impl Line {
    pub fn is_empty(&self, ctx: &StyleResolver) -> bool {
        let length = || self.start == self.end;
        let width = || self.width.unwrap_or(ctx.line_width()) <= 0.0;
        length() || width()
    }
}
