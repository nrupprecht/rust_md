use crate::core::particle::Particle;
use crate::core::vector::{Force, Position, Vector, Velocity};

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    /// The low and high bounds in each dimension.
    pub xlo: f64,
    pub xhi: f64,
    pub ylo: f64,
    pub yhi: f64,
}

impl Bounds {
    /// The width of the region represented by the SimData.
    pub fn width(&self) -> f64 {
        self.xhi - self.xlo
    }

    /// The height of the region represented by the SimData.
    pub fn height(&self) -> f64 {
        self.yhi - self.ylo
    }

    /// Checks whether a position falls within the bounds.
    pub fn is_in_bounds(&self, position: Position) -> bool {
        self.xlo <= position.x
            && position.x < self.xhi
            && self.ylo <=position.y
            && position.y <= self.yhi
    }
}

impl From<(f64, f64, f64, f64)> for Bounds {
    /// Create a bounds object from a quadruple of (xlo, xhi, ylo, yhi).
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Bounds { xlo: value.0, xhi: value.1, ylo: value.2, yhi: value.3 }
    }
}

/// Object that stores the fundamental data of the simulation.
pub struct SimData {
    /// The radius of each particle.
    pub radii: Vec<f64>,

    /// The mass of each particle.
    pub masses: Vec<f64>,

    /// The position of each particle.
    pub positions: Vec<Position>,

    /// The velocity of each particle.
    pub velocities: Vec<Velocity>,

    /// Buffer to accumulate the force on each particle.
    pub forces: Vec<Force>,

    /// The bounds of the SimData region.
    pub bounds: Bounds,

    /// A topology object, which is responsible for keeping particles in their canonical positions.
    pub topology: Box<dyn Topology>,

    /// The current simulation time.
    pub simulation_time: f64,
}

impl From<Bounds> for SimData {
    fn from(value: Bounds) -> Self {
        SimData::new(value.xlo, value.xhi, value.ylo, value.yhi)
    }
}

impl SimData {
    /// Create a new, empty, SimData, with the specified bounds.
    pub fn new(xlo: f64, xhi: f64, ylo: f64, yhi: f64) -> Self {
        SimData {
            radii: Vec::new(),
            masses: Vec::new(),
            positions: Vec::new(),
            velocities: Vec::new(),
            forces: Vec::new(),
            bounds: Bounds { xlo, xhi, ylo, yhi },
            topology: Box::new(HarmonicTopology{ wrap_x: true, wrap_y: true }),
            simulation_time: 0.0
        }
    }

    pub fn new_with_particles(bounds: Bounds, particles: &Vec<Particle>) -> Self {
        let mut sim_data = SimData::from(bounds);
        sim_data.add_particles(particles);
        sim_data
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
    pub fn width(&self) -> f64 {
        self.bounds.width()
    }

    /// The height of the region represented by the SimData.
    pub fn height(&self) -> f64 {
        self.bounds.height()
    }

    /// Add a particle, represented by a particle structure, to the SimData.
    /// ```
    /// let mut particles: Vec<Particle> = Vec::new();
    ///
    /// // Add some particles
    /// particles.push(Particle::new().with_position(Vector::new(1.2, 3.5)));
    /// particles.push(Particle::new().with_position(Vector::new(3.4, 6.2)));
    /// particles.push(Particle::new().with_position(Vector::new(7.1, 4.7)));
    ///
    /// // Create a SimData with specified bounds.
    /// let mut sim_data = SimData::new(0.0, 10.0, 0.0, 10.0);
    /// sim_data.add_particles(particles);
    /// ```
    pub fn add_particle(&mut self, particle: &Particle) -> &Self {
        self.radii.push(particle.radius);
        self.masses.push(particle.mass);
        self.positions.push(particle.position);
        self.velocities.push(particle.velocity);
        self.forces.push(particle.force);
        self
    }

    /// Add many particles to a SimData.
    pub fn add_particles(&mut self, particles: &Vec<Particle>) {
        for p in particles.iter() {
            self.radii.push(p.radius);
            self.masses.push(p.mass);
            self.positions.push(p.position);
            self.velocities.push(p.velocity);
            self.forces.push(Vector::zero());
        }
    }

    /// Get the distance squared between two particles.
    pub fn distance_sqr_between(&self, id1: usize, id2: usize) -> f64 {
        let r1 = self.positions[id1];
        let r2 = self.positions[id2];

        let dx = f64::abs(r1.x - r2.x);
        let dx = f64::min(dx, f64::abs(dx - self.width()));

        let dy = f64::abs(r1.y - r2.y);
        let dy = f64::min(dy, f64::abs(dy - self.height()));

        dx * dx + dy * dy
    }

    /// Set all particles' positions to be their canonical positions.
    pub fn canonical_positions(&mut self) {
        for i in 0 .. self.num_particles() {
            let p: &mut Position = &mut self.positions[i];
            self.topology.canonical_position(&mut p.x, &mut p.y, &self.bounds);
        }
    }
}

pub trait Topology {
    /// Take a particle in the sim data an put them in their canonical positions. For example,
    /// if the particles are on a torus (harmonic boundary conditions), and a particle goes beyond
    /// the "edge" of the simulation, canonical_positions will "wrap" the particle back so it appears
    /// on the other side of the simulation.
    fn canonical_position(&self, x: &mut f64, y: &mut f64, bounds: &Bounds);
}

struct OpenTopology {}

struct HarmonicTopology {
    wrap_x: bool,
    wrap_y: bool,
}

impl Topology for OpenTopology {
    fn canonical_position(&self, x: &mut f64, y: &mut f64, bounds: &Bounds) {}
}

impl Topology for HarmonicTopology {
    fn canonical_position(&self, x: &mut f64, y: &mut f64, bounds: &Bounds) {
        if self.wrap_x {
            while *x < bounds.xlo {
                *x += bounds.width();
            }
            while bounds.xhi < *x {
                *x -= bounds.width();
            }
        }

        if self.wrap_y {
            while *y < bounds.ylo {
                *y += bounds.height();
            }
            while bounds.yhi < *y {
                *y -= bounds.height()
            }
        }

        assert!(bounds.is_in_bounds(Vector::new(*x, *y)));
    }
}

// =================================================================================================
//  Unit Tests.
// =================================================================================================

#[cfg(test)]
mod tests {
    use crate::assert_close;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_bounds() {
        let bounds = Bounds {
            xlo: 0.0,
            xhi: 2.5,
            ylo: -2.0,
            yhi: 3.25,
        };
        assert_eq!(bounds.width(), 2.5);
        assert_eq!(bounds.height(), 5.25);
    }

    #[test]
    fn test_simdata_construction_from_bounds() {
        let bounds = Bounds {
            xlo: 0.0,
            xhi: 2.5,
            ylo: -2.0,
            yhi: 2.0,
        };
        let simdata = SimData::from(bounds);
        assert_eq!(simdata.bounds.xlo, 0.0);
        assert_eq!(simdata.bounds.xhi, 2.5);
        assert_eq!(simdata.bounds.ylo, -2.0);
        assert_eq!(simdata.bounds.yhi, 2.0);

        assert_eq!(simdata.width(), bounds.width());
        assert_eq!(simdata.height(), bounds.height());
    }

    #[test]
    fn test_harmonic_topology() {
        let topology = HarmonicTopology{wrap_x: true, wrap_y: true};
        let bounds = Bounds::from((0.0, 10.0, 0.0, 10.0));

        {
            let mut pos = Position::new(-1.0, 3.2);
            topology.canonical_position(&mut pos.x, &mut pos.y, &bounds);
            assert_close!(pos.x, 9.0, 1.0e-6);
            assert_close!(pos.y, 3.2, 1.0e-6);
        }
        {
            let mut pos = Position::new(3.6, 3.2);
            topology.canonical_position(&mut pos.x, &mut pos.y, &bounds);
            assert_close!(pos.x, 3.6, 1.0e-6);
            assert_close!(pos.y, 3.2, 1.0e-6);
        }
        {
            let mut pos = Position::new(10.92, 4.0);
            topology.canonical_position(&mut pos.x, &mut pos.y, &bounds);
            assert_close!(pos.x, 0.92, 1.0e-6);
            assert_close!(pos.y, 4.0, 1.0e-6);
        }
        {
            let mut pos = Position::new(0.3, -0.2);
            topology.canonical_position(&mut pos.x, &mut pos.y, &bounds);
            assert_close!(pos.x, 0.3, 1.0e-6);
            assert_close!(pos.y, 9.8, 1.0e-6);
        }
        {
            let mut pos = Position::new(11.1, 3.2);
            topology.canonical_position(&mut pos.x, &mut pos.y, &bounds);
            assert_close!(pos.x, 1.1, 1.0e-6);
            assert_close!(pos.y, 3.2, 1.0e-6);

        }
    }

    #[test]
    fn test_simdata_canonical_positions() {

    }
}
