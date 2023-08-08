use crate::core::simdata::SimData;
use crate::core::universe::Universe;
use crate::core::vector::Velocity;

// ======================================================
//  Define modules.
// ======================================================

pub mod velocity_verlet;
pub mod overdamped;


/// The integrator trait represents objects that can integrate the particles in a sim data, potentially including
/// positions, velocities, angle, angular velocity, etc.
///
pub trait Integrator {
    fn get_timestep(&self) -> f64;
    fn pre_forces(&mut self, sim_data: &mut SimData);
    fn post_forces(&mut self, sim_data: &mut SimData);
    fn post_step(&mut self, sim_data: &mut SimData);
}


// =================================================================================================
//  Unit Tests.
// =================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_positions() {}
}