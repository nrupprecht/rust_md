use std::ops;

/// Represents a two-dimensional vector.
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    /// Create a copy of the zero vector.
    pub fn zero() -> Vector {
        Vector { x: 0., y: 0. }
    }

    /// Create a new vector with a specified x and y position.
    ///
    /// The statement
    /// ```
    /// let v = Vector::new(1.0, 3.6);
    /// ```
    /// is equivalent to
    /// ```
    /// let v = Vector{ x: 1.0, y: 3.6 };
    /// ```
    pub fn new(x: f64, y: f64) -> Vector {
        Vector{x, y}
    }

    /// Get the length squared of the vector.
    pub fn length_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Get the length (L2 norm) of the vector.
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_sqr())
    }

    /// Get a unit vector in the same direction as a given vector. If the vector is the zero vector,
    /// returns the zero vector.
    pub fn normalize(v: Vector) -> Vector {
        if v.x == 0.0 && v.y == 0.0 {
            return v;
        }
        v / v.length()
    }

    /// Return the x and y components of the vector as a tuple of mutable floats.
    pub fn as_mut_tuple(&mut self) -> (&mut f64, &mut f64) {
        (&mut self.x, &mut self.y)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub type Position = Vector;
pub type Velocity = Vector;
pub type Force = Vector;

// =================================================================================================
//  Unit Tests.
// =================================================================================================
