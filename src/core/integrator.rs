use crate::core::simdata::SimData;
use crate::core::universe::Universe;
use crate::core::vector::Velocity;

/// The integrator trait represents objects that can integrate the particles in a sim data, potentially including
/// positions, velocities, angle, angular velocity, etc.
///
pub trait Integrator {
    fn get_timestep(&self) -> f64;
    fn pre_forces(&mut self, sim_data: &mut SimData);
    fn post_forces(&mut self, sim_data: &mut SimData);
    fn post_step(&mut self, sim_data: &mut SimData);
}


pub struct VelocityVerlet {
    pub(crate) dt: f64
}

impl Integrator for VelocityVerlet {
    fn get_timestep(&self) -> f64 {
        self.dt
    }

    fn pre_forces(&mut self, sim_data: &mut SimData) {
        // First half kick.
        self.update_velocities(sim_data);
        self.update_positions(sim_data);
    }

    fn post_forces(&mut self, sim_data: &mut SimData) {
        // Second half kick.
        self.update_velocities(sim_data);
    }

    fn post_step(&mut self, sim_data: &mut SimData) {
        sim_data.simulation_time += self.dt;
    }
}

impl VelocityVerlet {
    fn update_positions(&mut self, sim_data: &mut SimData) {
        for i in 0..sim_data.num_particles() {
            sim_data.positions[i].x += sim_data.velocities[i].x * self.dt;
            sim_data.positions[i].y += sim_data.velocities[i].y * self.dt;
        }

        // Make sure particles stay in their canonical positions.
        sim_data.canonical_positions();
    }

    fn update_velocities(&mut self, sim_data: &mut SimData) {
        let hdt = self.dt / 2.0;
        for i in 0..sim_data.num_particles() {
            let im = 1.0 / sim_data.masses[i];
            sim_data.velocities[i].x += sim_data.forces[i].x * hdt * im;
            sim_data.velocities[i].y += sim_data.forces[i].y * hdt * im;
        }
    }
}


// =================================================================================================
//  Unit Tests.
// =================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_positions() {

    }
}