use crate::core::simdata::SimData;
use crate::core::vector::Position;

/// Defines an implementation of an object that periodically gathers data from a SimData for the purpose
/// of gathering statistics about the simulation.
pub trait Monitor {
    fn pre_step(&mut self, sim_data: &SimData) {}
    fn pre_forces(&mut self, sim_data: &SimData) {}
    fn post_forces(&mut self, sim_data: &SimData) {}
    fn post_step(&mut self, sim_data: &SimData) {}

    fn test(&self) {}
}

pub struct PositionMonitor {
    /// The times at which snapshots are taken.
    pub times: Vec<f64>,
    /// The positions of all the particles at each time slice.
    pub positions: Vec<Vec<Position>>,

    /// Time between snapshots being take.
    pub snapshot_delay: f64,

    /// The last time at which a snapshot was taken.
    last_snapshot_time: Option<f64>,
}

impl PositionMonitor {
    pub fn new(snapshot_delay: f64) -> PositionMonitor {
        PositionMonitor {
            times: vec![],
            positions: vec![],
            snapshot_delay,
            last_snapshot_time: None
        }
    }
}

impl Monitor for PositionMonitor {
    /// If this is the first timestep, or enough time has gone by, save the positions of all the particles.
    fn post_step(&mut self, sim_data: &SimData) {
        if self.last_snapshot_time.is_none()
            || self.snapshot_delay < sim_data.simulation_time - self.last_snapshot_time.unwrap() {
            let mut new_positions = Vec::new();
            for i in 0..sim_data.num_particles() {
                new_positions.push(sim_data.positions[i]);
            }
            self.positions.push(new_positions);
        }
    }
}