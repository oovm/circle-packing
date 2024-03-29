use crate::{π, Ellipse, Float, Point};
use core::ops::Div;

impl Ellipse {
    /// Create a new ellipse with the coefficient of equation.
    ///
    /// ```math
    /// A x^2 + B y^2 + C xy + D x + E y + F = 0
    /// ```
    pub fn from_coefficient(a: Float, b: Float, c: Float, d: Float, e: Float, f: Float) -> Self {
        Self { a, b: b.div(2.0), c, d: d.div(2.0), e: e.div(2.0), f }
    }
    /// Create a new ellipse with the center and the two axes and rotate.
    pub fn from_transform(center: Point, major: Float, minor: Float, rotate: Float) -> Self {
        let rotate_rad = rotate.to_radians();
        let cos_theta = rotate_rad.cos();
        let sin_theta = rotate_rad.sin();

        let a = major.powi(2) * sin_theta.powi(2) + minor.powi(2) * cos_theta.powi(2);
        let b = major.powi(2) * sin_theta * cos_theta - minor.powi(2) * sin_theta * cos_theta;
        let c = major.powi(2) * cos_theta.powi(2) + minor.powi(2) * sin_theta.powi(2);
        let d = -2.0 * a * center.x - b * center.y;
        let e = -b * center.x - 2.0 * c * center.y;
        let f = a * center.x.powi(2) + b * center.x * center.y + c * center.y.powi(2) - major.powi(2) * minor.powi(2);

        Self { a, b, c, d, e, f }
    }
    /// Create a new ellipse with 5 points.
    pub fn from_5_points(p1: Point, p2: Point, p3: Point, p4: Point, p5: Point) {
        todo!()
    }
}

impl Ellipse {
    /// Get the major axis of the ellipse.
    pub fn major_axis(&self) -> Float {
        self.a
    }
    /// Get the minor axis of the ellipse.
    pub fn minor_axis(&self) -> Float {
        self.b
    }
    /// Get the major delta of the ellipse.
    /// ```math
    /// \Delta =
    /// \begin{vmatrix}
    /// A & B & D \\
    /// B & C & E \\
    /// D & E & F \\
    /// \end{vmatrix}
    /// = ACF+2BDE-AE^2-CD^2-FB^2
    /// ```
    pub fn major_delta(&self) -> Float {
        let Self { a, b, c, d, e, f } = self;
        a * c * f + 2.0 * b * d * e - a * e * e - c * d * d - f * b * b
    }
    /// Get the minor delta of the ellipse.
    /// ```math
    /// \delta =
    /// \begin{vmatrix}
    /// A & B \\
    /// B & C \\
    /// \end{vmatrix}
    /// = AC - B^2
    /// ```
    pub fn minor_delta(&self) -> Float {
        let Self { a, b, c, d: _, e: _, f: _ } = self;
        a * c - b * b
    }
    /// Get the rotation of the ellipse.
    ///
    ///
    /// ```math
    /// \alpha=
    /// \begin{cases}
    /// \dfrac{1}{2}\arctan\dfrac{B}{A-C} &,  A \ne C\\
    /// \dfrac{\pi}{4} &, A = C\\
    /// \end{cases}
    /// ```
    pub fn rotate(&self) -> Float {
        let Self { a, b, c, d: _, e: _, f: _ } = self;
        match a.eq(b) {
            true => π / 4.0,
            false => (b.div(a - c)).atan() / 2.0,
        }
    }
    /// Get the center of the ellipse.
    pub fn center(&self) -> Point {
        let Self { a, b, c, d, e, f: _ } = self;
        let x = (b * e - c * d) / self.minor_delta();
        let y = (b * d - a * e) / self.minor_delta();
        Point { x, y }
    }
    /// Return the parameters.
    /// ```math
    /// Ax^2+2Bxy+Cy^2+2Dx+2Ey+F=0
    /// ```
    pub fn parameter(&self) -> (Float, Float, Float, Float, Float, Float) {
        (self.a, self.b, self.c, self.d, self.e, self.f)
    }
    /// Get the parameter of the ellipse.
    ///
    /// ```math
    /// Q = \begin{bmatrix}
    /// A & B & D \\
    /// B & C & E \\
    /// D & E & F \\
    /// \end{bmatrix}
    /// ```
    pub fn parameter_matrix(&self) -> [[Float; 3]; 3] {
        [[self.a, self.b, self.d], [self.d, self.c, self.e], [self.d, self.e, self.f]]
    }
    /// Return the major axis of the ellipse.
    pub fn transform(&self) -> (Point, Float, Float, Float) {
        todo!()
    }
    /// Return the major axis of the ellipse.
    pub fn transform_matrix(&self) -> [[Float; 3]; 3] {
        todo!()
    }
}
