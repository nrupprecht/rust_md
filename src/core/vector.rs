use std::ops;

/// Represents a two-dimensional vector.
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    /// Create a copy of the zero vector.
    pub fn zero() -> Vector {
        Vector { x: 0., y: 0. }
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

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub type Position = Vector;
pub type Velocity = Vector;
pub type Force = Vector;

// =================================================================================================
//  Unit Tests.
// =================================================================================================
