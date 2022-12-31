use crate::core::simdata::SimData;
use crate::core::vector::Vector;

pub trait Force {
    fn calculate_forces(&self, sim_data: &mut SimData, id1: usize, id2: usize);
}

pub struct HardSphereForce {
    pub(crate) repulsion: f64
}

pub fn force_loop<Iterable>(force: &Box<dyn Force>, sim_data: &mut SimData, iterable: Iterable)
    where Iterable: IntoIterator<Item=(usize, usize)>
{
    // Clear the buffer of forces.
    for f in sim_data.forces.iter_mut() {
        f.x = 0.0;
        f.y = 0.0;
    }

    for (id1, id2) in iterable.into_iter() {
        force.calculate_forces(sim_data, id1, id2);
    }
}

impl Force for HardSphereForce {
    fn calculate_forces(&self, sim_data: &mut SimData, id1: usize, id2: usize) {

        let rsqr = sim_data.distance_sqr_between(id1, id2);
        let sum_radii = sim_data.radii[id1] + sim_data.radii[id2];
        if rsqr < sum_radii * sum_radii {
            // Calculate the magnitude of the force.
            let overlap = sum_radii - f64::sqrt(sum_radii);

            let displacement = sim_data.positions[id2] - sim_data.positions[id1];
            let unit = Vector::normalize(displacement);

            sim_data.forces[id1] -= unit * self.repulsion * overlap;
            sim_data.forces[id2] += unit * self.repulsion * overlap;
        }

    }
}