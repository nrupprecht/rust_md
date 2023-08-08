use crate::core::integrator::Integrator;
use crate::core::simdata::SimData;

pub struct OverdampedIntegrator {
    pub dt: f64,
    pub damping_constant: f64
}

impl Integrator for OverdampedIntegrator {
    fn get_timestep(&self) -> f64 {
        self.dt
    }

    fn pre_forces(&mut self, sim_data: &mut SimData) {}

    fn post_forces(&mut self, sim_data: &mut SimData) {
        for i in 0..sim_data.num_particles() {
            let im = 1.0 / sim_data.masses[i];
            sim_data.positions[i].x += sim_data.forces[i].x * self.dt * im;
            sim_data.positions[i].y += sim_data.forces[i].y * self.dt * im;
        }

        // Make sure particles stay in their canonical positions.
        sim_data.canonical_positions();
    }

    fn post_step(&mut self, sim_data: &mut SimData) {
        sim_data.simulation_time += self.dt;
    }
}

impl OverdampedIntegrator {
    pub fn new(dt: f64, damping_constant: f64) -> OverdampedIntegrator {
        OverdampedIntegrator { dt, damping_constant }
    }
}