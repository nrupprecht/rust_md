use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use crate::core::force::{Force, HardSphereForce, force_loop};
use crate::core::simdata::{Bounds, SimData};
use crate::core::integrator::{Integrator, velocity_verlet::VelocityVerlet};
use crate::core::verlet_lists::create_verlet_lists;
use crate::core::monitor::{Monitor, PositionMonitor};

use std::time::{Duration, Instant};
use crate::core::integrator::overdamped::OverdampedIntegrator;

pub struct Universe {
    pub sim_data: SimData,
    pub integrator: Box<dyn Integrator>,
    pub forces: Box<dyn Force>,

    pub is_running: bool,
    pub iterations: i64,

    pub monitors: HashMap<String, Box<dyn Monitor>>,

    max_time: Option<f64>,
    max_iterations: Option<i64>,

    integrator_time: u128,
    forces_time: u128,
    verlet_lists_time: u128,
    total_time: u128,
}

impl Universe {
    pub fn new(bounds: Bounds) -> Universe {
        Universe {
            sim_data: SimData::from(bounds),
            integrator: Box::new(VelocityVerlet {
                dt: 0.001
            }),
            forces: Box::new(HardSphereForce {
                repulsion: 100.0
            }),
            is_running: true,
            iterations: 0,
            monitors: HashMap::new(),
            max_time: None,
            max_iterations: None,

            integrator_time: 0,
            forces_time: 0,
            verlet_lists_time: 0,
            total_time: 0,
        }
    }

    pub fn with_simdata(&mut self, sim_data: SimData) -> &mut Self {
        self.sim_data = sim_data;
        self
    }

    pub fn with_forces(&mut self, force: Box<dyn Force>) -> &mut Self {
        self.forces = force;
        self
    }

    pub fn with_integrator(&mut self, integrator: Box<dyn Integrator>) -> &mut Self {
        self.integrator = integrator;
        self
    }

    /// Add a monitoring object to the universe.
    pub fn add_monitor(&mut self, name: &str, monitor: Box<dyn Monitor>) {
        self.monitors.insert(name.to_string(), monitor);
    }

    /// Get a monitoring object from the universe. If none exists, None is returned.
    pub fn get_monitor(&self, name: &str) -> Option<&Box<dyn Monitor>> {
        self.monitors.get(name)
    }

    pub fn get_sim_data(&mut self) -> &mut SimData {
        &mut self.sim_data
    }

    pub fn get_integrator(&mut self) -> &dyn Integrator {
        self.integrator.deref()
    }

    pub fn get_forces(&mut self) -> &dyn Force {
        self.forces.deref()
    }

    fn run(&mut self) {
        // Make sure all particles start out in their canonical positions.
        self.sim_data.canonical_positions();

        let start_time = Instant::now();
        while self.is_running {
            println!("Iteration {}, t = {}. There are {} particles.",
                     self.iterations,
                     self.sim_data.simulation_time,
                     self.sim_data.num_particles());

            self.pre_step();

            self.pre_forces();

            self.forces();

            self.post_forces();

            self.post_step();

            // Update iteration count.
            self.iterations += 1;

            // Check termination conditions.
            if let Some(max_time) = self.max_time {
                if max_time < self.sim_data.simulation_time {
                    self.is_running = false;
                }
            }
            if let Some(max_iterations) = self.max_iterations {
                if max_iterations <= self.iterations {
                    self.is_running = false;
                }
            }
        }
        self.total_time = start_time.elapsed().as_nanos();
    }

    pub fn run_until(&mut self, time: f64) {
        self.max_time = Some(time);
        self.run();
    }

    pub fn relax_for(&mut self, time: f64) {
        // let relaxer = Universe::new(self.sim_data.bounds)
        //     .with_simdata(self.sim_data.clone())
        //     .with_forces(self.forces.clone())
        //     .with_integrator(Box::new(OverdampedIntegrator::new(0.001, 5.0)));

        // TODO(Nate): Finish.
    }

    fn pre_step(&mut self) {
        // Run all monitor objects.
        for (_, monitor) in self.monitors.iter_mut() {
            monitor.pre_step(&self.sim_data);
        }
    }

    fn pre_forces(&mut self) {
        let ig_now = Instant::now();
        self.integrator.pre_forces(&mut self.sim_data);
        self.integrator_time += ig_now.elapsed().as_nanos();

        // Run all monitor objects.
        for (_, monitor) in self.monitors.iter_mut() {
            monitor.pre_forces(&mut self.sim_data);
        }
    }

    fn forces(&mut self) {
        let vl_now = Instant::now();
        let verlet_lists = create_verlet_lists(&mut self.sim_data, 0.1);
        self.verlet_lists_time += vl_now.elapsed().as_nanos();

        let fl_now = Instant::now();
        force_loop(self.forces.deref(), &mut self.sim_data, &verlet_lists);
        self.forces_time += fl_now.elapsed().as_nanos();
    }

    fn post_forces(&mut self) {
        self.integrator.post_forces(&mut self.sim_data);

        // Run all monitor objects.
        for (_, monitor) in self.monitors.iter_mut() {
            monitor.post_forces(&mut self.sim_data);
        }
    }

    fn post_step(&mut self) {
        self.integrator.post_step(&mut self.sim_data);

        // Run all monitor objects.
        for (_, monitor) in self.monitors.iter_mut() {
            monitor.post_step(&mut self.sim_data);
        }
    }
}