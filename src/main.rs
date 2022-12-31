use std::cell::RefCell;
use std::rc::Rc;
use plotly;
use plotly::common::Mode;
use plotly::{Layout, Plot, Scatter};

use rand::Rng;
use crate::core::particle::Particle;
use crate::core::simdata::{Bounds, SimData};
use crate::core::force::Force;
use crate::core::monitor::PositionMonitor;
use crate::core::universe::Universe;

pub mod core;
pub mod utils;

use crate::core::vector::{Position, Vector, Velocity};
use crate::core::verlet_lists::create_verlet_lists;

fn generate_particles() -> Vec<Particle>{
    let mut rng = rand::thread_rng();
    let mut random = || rng.gen_range(0.0..10.0);

    let mut particles = Vec::new();
    for _ in 0..10 {
        let p = Particle::new()
            .with_position(Vector::new(random(), random()))
            .with_velocity(Vector::new(random() - 5.0, random() - 5.0));
        particles.push(p);
    }
    particles
}

fn specific_scenario() -> Vec<Particle> {
    let mut particles = Vec::new();
    particles.push(Particle::new().with_position(Vector::new(6.446288539458056, 6.217110127096928)).with_velocity(Vector::new(-4.407848524198707, 3.6995346746413134)));
    particles.push(Particle::new().with_position(Vector::new(6.294063113202821, 9.164060403351451)).with_velocity(Vector::new(-3.3529670672928336, 3.455264102358342)));
    particles.push(Particle::new().with_position(Vector::new(5.2501633111388095, 6.756661016465184)).with_velocity(Vector::new(1.8355446297693963, 0.6900124402930423)));
    particles.push(Particle::new().with_position(Vector::new(1.9520727230736101, 9.617699811943838)).with_velocity(Vector::new(-3.4658370935872185, 2.9162615067827495)));
    particles.push(Particle::new().with_position(Vector::new(6.891032536613626, 7.272656589024029)).with_velocity(Vector::new(0.7620983716169505, 3.4213059428926798)));
    particles.push(Particle::new().with_position(Vector::new(8.772348654700451, 7.040637761906032)).with_velocity(Vector::new(-1.744660216621523, -2.174623389581567)));
    particles.push(Particle::new().with_position(Vector::new(1.7275232232347149, 1.5405706994551838)).with_velocity(Vector::new(4.088895874634694, -3.0403902851946674)));
    particles.push(Particle::new().with_position(Vector::new(3.094174157733802, 3.6138067778299576)).with_velocity(Vector::new(-4.943446795030946, 4.199995443461713)));
    particles.push(Particle::new().with_position(Vector::new(6.651726140957884, 9.262470253887123)).with_velocity(Vector::new(2.5922623665517435, -4.134675846003271)));
    particles.push(Particle::new().with_position(Vector::new(4.557260954842059, 7.77621971951792)).with_velocity(Vector::new(-1.0911654157490402, -1.662858835103338)));
    particles
}

fn main() {

    let particles = specific_scenario();


    println!("let mut particles = Vec::new();");
    for p in particles.iter() {
        println!("particles.push(Particle::new().with_position(Vector::new({}, {})).with_velocity(Vector::new({}, {})));", p.position.x, p.position.y, p.velocity.x, p.velocity.y);
    }

    let sim_bounds = Bounds::from((0., 10., 0., 10.));
    let mut universe = Universe::new(sim_bounds);

    let positions = Box::new(PositionMonitor::new(1.0 / 30.0));
    universe.add_monitor(positions);

    universe.sim_data.add_particles(&particles);
    universe.run();

    let mut x = Vec::new();
    let mut y = Vec::new();
    for i in 0..universe.sim_data.num_particles() {
        x.push(universe.sim_data.positions[i].x);
        y.push(universe.sim_data.positions[i].y);
    }
    let trace = Scatter::new(x, y).name("My trace").mode(Mode::Markers);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    let layout = Layout::new().title("<b>Line and Scatter Plot</b>".into());
    plot.set_layout(layout);
    plot.show();
}
