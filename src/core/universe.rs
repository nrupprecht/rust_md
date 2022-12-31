use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
use crate::core::force::{Force, HardSphereForce, force_loop};
use crate::core::simdata::{Bounds, SimData};
use crate::core::integrator::{Integrator, VelocityVerlet};
use crate::core::verlet_lists::create_verlet_lists;
use crate::core::monitor::{Monitor, PositionMonitor};

pub struct Universe {
    pub sim_data: SimData,
    pub integrator: Box<dyn Integrator>,
    pub forces: Box<dyn Force>,

    pub is_running: bool,
    pub iterations: i64,

    pub monitors: Vec<Box<dyn Monitor>>,
}

impl Universe {
    pub fn new(bounds: Bounds) -> Universe {
        Universe {
            sim_data: SimData::from(bounds),
            integrator: Box::new(VelocityVerlet {
                dt: 0.001
            }),
            forces: Box::new(HardSphereForce {
                repulsion: 10.0
            }),
            is_running: true,
            iterations: 0,
            monitors: Vec::new()
        }
    }

    /// Add a monitoring object to the universe.
    pub fn add_monitor(&mut self, monitor: Box<dyn Monitor>) {
        self.monitors.push(monitor);
    }

    pub fn run(&mut self) {
        // Make sure all particles start out in their canonical positions.
        self.sim_data.canonical_positions();

        while self.is_running {
            println!("Iteration {}, t = {}", self.iterations, self.sim_data.simulation_time);

            if self.iterations == 131 {
                println!("Here");
            }

            self.pre_step();

            self.pre_forces();

            self.forces();

            self.post_forces();

            self.post_step();

            self.iterations += 1;
        }
    }

    fn pre_step(&mut self) {
        // Run all monitor objects.
        for monitor in self.monitors.iter_mut() {
            monitor.pre_step(&self.sim_data);
        }
    }

    fn pre_forces(&mut self) {
        self.integrator.pre_forces(&mut self.sim_data);

        // Run all monitor objects.
        for monitor in self.monitors.iter_mut() {
            monitor.pre_forces(&self.sim_data);
        }
    }

    fn forces(&mut self) {
        let verlet_lists = create_verlet_lists(&self.sim_data, 0.1);
        force_loop(&self.forces, &mut self.sim_data, &verlet_lists);
    }

    fn post_forces(&mut self) {
        self.integrator.post_forces(&mut self.sim_data);

        // Run all monitor objects.
        for monitor in self.monitors.iter_mut() {
            monitor.post_forces(&self.sim_data);
        }
    }

    fn post_step(&mut self) {
        self.integrator.post_step(&mut self.sim_data);

        // Run all monitor objects.
        for monitor in self.monitors.iter_mut() {
            monitor.post_step(&self.sim_data);
        }
    }
}