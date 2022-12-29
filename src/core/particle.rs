use crate::core::vector::{Force, Position, Vector, Velocity};

/// A structure that represents a single particle.
pub struct Particle {
    pub position: Position,
    pub radius: f32,
    pub mass: f32,
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
    pub fn set_position(&mut self, position: Vector) -> &mut Self {
        self.position = position;
        self
    }

    /// Set the radius of a particle. Allows for chaining.
    pub fn set_radius(&mut self, r: f32) -> &mut Self {
        self.radius = r;
        self
    }

    /// Set the mass of a particle. Allows for chaining.
    pub fn set_mass(&mut self, mass: f32) -> &mut Self {
        self.mass = mass;
        self
    }

    /// Set the velocity of a particle. Allows for chaining.
    pub fn set_velocity(&mut self, velocity: Velocity) -> &mut Self {
        self.velocity = velocity;
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
        let mut particle = Particle::new();
        particle
            .set_radius(2.34)
            .set_velocity(Velocity { x: 1.2, y: 2.5 })
            .set_mass(1.77);
        assert_eq!(particle.radius, 2.34);
        assert_eq!(particle.velocity.x, 1.2);
        assert_eq!(particle.velocity.y, 2.5);
        assert_eq!(particle.mass, 1.77);
    }
}
