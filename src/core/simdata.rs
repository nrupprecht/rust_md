use crate::core::particle::Particle;
use crate::core::vector::{Vector, Position, Velocity, Force};

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    /// The low and high bounds in each dimension.
    pub xlo: f32, pub xhi: f32, pub ylo: f32, pub yhi: f32
}

impl Bounds {
    /// The width of the region represented by the SimData.
    pub fn width(&self) -> f32 {
        self.xhi - self.xlo
    }

    /// The height of the region represented by the SimData.
    pub fn height(&self) -> f32 {
        self.yhi - self.ylo
    }
}

/// Object that stores the fundamental data of the simulation.
#[derive(Debug, Clone)]
pub struct SimData {
    /// The radius of each particle.
    pub radii: Vec<f32>,

    /// The mass of each particle.
    pub masses: Vec<f32>,

    /// The position of each particle.
    pub positions: Vec<Position>,

    /// The velocity of each particle.
    pub velocities: Vec<Velocity>,

    /// Buffer to accumulate the force on each particle.
    pub forces: Vec<Force>,

    /// The bounds of the SimData region.
    pub bounds: Bounds,
}

impl From<Bounds> for SimData {
    fn from(value: Bounds) -> Self {
        SimData::new(value.xlo, value.xhi, value.ylo, value.yhi)
    }
}

impl SimData {
    /// Create a new, empty, SimData, with the specified bounds.
    pub fn new(xlo: f32, xhi: f32, ylo: f32, yhi: f32) -> Self {
        SimData {
            radii: vec![],
            masses: vec![],
            positions: vec![],
            velocities: vec![],
            forces: vec![],
            bounds: Bounds{ xlo, xhi, ylo, yhi}
        }
    }

    pub fn new_with_particles(bounds: Bounds, particles: Vec<Particle>) -> Self {
        let mut simdata = SimData::from(bounds);
        for p in particles.iter() {
            simdata.radii.push(p.radius);
            simdata.masses.push(p.mass);
            simdata.positions.push(p.position);
            simdata.velocities.push(p.velocity);
            simdata.forces.push(Vector::zero());
        }
        simdata
    }

    /// Return the number of particles.
    pub fn num_particles(&self) -> usize {
        self.radii.len()
    }

    /// Check whether there is no data in the SimData.
    pub fn is_empty(&self) -> bool {
        self.radii.is_empty()
    }

    /// The width of the region represented by the SimData.
    pub fn width(&self) -> f32 {
        self.bounds.width()
    }

    /// The height of the region represented by the SimData.
    pub fn height(&self) -> f32 {
        self.bounds.height()
    }

    /// Add a particle, represented by a particle structure, to the SimData.
    pub fn add_particle(&mut self, particle: &Particle) -> &Self {
        self.radii.push(particle.radius);
        self.masses.push(particle.mass);
        self.positions.push(particle.position);
        self.velocities.push(particle.velocity);
        self.forces.push(particle.force);
        self
    }

    /// Get the distance squared between two particles.
    pub fn distance_sqr_between(&self, id1: usize, id2: usize) -> f32 {
        let r1 = self.positions[id1];
        let r2 = self.positions[id2];

        let dx = f32::abs(r1.x - r2.x);
        let dx = f32::min(dx, f32::abs(dx - self.width()));

        let dy = f32::abs(r1.y - r2.y);
        let dy = f32::min(dy, f32::abs(dy - self.height()));

        dx * dx + dy * dy
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
    fn test_bounds() {
        let bounds = Bounds{ xlo: 0.0, xhi: 2.5, ylo: -2.0, yhi: 3.25};
        assert_eq!(bounds.width(), 2.5);
        assert_eq!(bounds.height(), 5.25);
    }

    #[test]
    fn test_simdata_construction_from_bounds() {
        let bounds = Bounds{ xlo: 0.0, xhi: 2.5, ylo: -2.0, yhi: 2.0};
        let simdata = SimData::from(bounds);
        assert_eq!(simdata.bounds.xlo, 0.0);
        assert_eq!(simdata.bounds.xhi, 2.5);
        assert_eq!(simdata.bounds.ylo, -2.0);
        assert_eq!(simdata.bounds.yhi, 2.0);

        assert_eq!(simdata.width(), bounds.width());
        assert_eq!(simdata.height(), bounds.height());
    }
}