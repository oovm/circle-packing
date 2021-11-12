use crate::{Circle, Ellipse, Point};
use core::fmt::Write;
use latexify::Latexify;

impl Latexify for Point {
    type Context = ();

    fn fmt<W: Write>(&self, _: &Self::Context, f: &mut W) -> core::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Latexify for Circle {
    type Context = ();

    fn fmt<W: Write>(&self, _: &Self::Context, f: &mut W) -> core::fmt::Result {
        write!(f, "(x-{})^2+(y-{})^2 = {}^2", self.center.x, self.center.y, self.radius)
    }
}
impl Latexify for Ellipse {
    type Context = ();

    fn fmt<W: Write>(&self, _: &Self::Context, f: &mut W) -> core::fmt::Result {
        write!(f, "{}x^2+{}y^2+{}xy+{}x+{}y+{}=0", self.a, self.b * 2.0, self.c, self.d * 2.0, self.e * 2.0, self.f)
    }
}
