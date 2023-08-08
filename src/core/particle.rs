use crate::core::vector::{Force, Position, Vector, Velocity};

/// A structure that represents a single particle.
#[derive(Copy, Clone)]
pub struct Particle {
    pub position: Position,
    pub radius: f64,
    pub mass: f64,
    pub velocity: Velocity,

    pub force: Force,
}

impl Particle {
    /// Create a new particle, with default information.
    pub fn new() -> Self {
        Particle {
            position: Vector::zero(),
            radius: 1.,
            mass: 1.,
            velocity: Vector::zero(),
            force: Vector::zero(),
        }
    }

    /// Set the position of a particle. Allows for chaining.
    pub fn with_position(&mut self, position: Vector) -> &mut Self {
        self.position = position;
        self
    }

    /// Set the x and y coordinates of the particle. Allows for chaining.
    pub fn with_coords(&mut self, x: f64, y: f64) -> &mut Self {
        self.position.x = x;
        self.position.y = y;
        self
    }

    /// Set the radius of a particle. Allows for chaining.
    pub fn with_radius(&mut self, r: f64) -> &mut Self {
        self.radius = r;
        self
    }

    /// Set the mass of a particle. Allows for chaining.
    pub fn with_mass(&mut self, mass: f64) -> &mut Self {
        self.mass = mass;
        self
    }

    /// Set the velocity of a particle. Allows for chaining.
    pub fn with_velocity(&mut self, velocity: Velocity) -> &mut Self {
        self.velocity = velocity;
        self
    }

    /// Set the components of velocity. Allows for chaining.
    pub fn with_velocity_components(&mut self, x: f64, y: f64) -> &mut Self {
        self.velocity.x = x;
        self.velocity.y = y;
        self
    }

    pub fn with_density(&mut self, density: f64) -> &mut Self {
        let area = std::f64::consts::PI * self.radius * self.radius;
        let mut p = *self;
        self.mass = density * area;
        self
    }
}

// =================================================================================================
//  Unit Tests.
// =================================================================================================

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_particle_construction() {
        let particle = Particle::new()
            .with_radius(2.34)
            .with_velocity(Velocity::new(1.2, 2.5))
            .with_mass(1.77);
        assert_eq!(particle.radius, 2.34);
        assert_eq!(particle.velocity.x, 1.2);
        assert_eq!(particle.velocity.y, 2.5);
        assert_eq!(particle.mass, 1.77);
    }
}
