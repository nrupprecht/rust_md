use plotly;
use plotly::common::Mode;
use plotly::{Layout, Plot, Scatter};

use rand::Rng;
use crate::core::particle::Particle;
use crate::core::simdata::SimData;

pub mod core;

use crate::core::vector::{Vector, Position, Velocity};
use crate::core::verlet_lists::create_verlet_lists;


#[derive(Debug)]
struct Universe {
    sim_data: SimData,
}


fn main() {
    let mut rng = rand::thread_rng();
    let mut random = || rng.gen_range(0.0..10.0);

    let mut particles = Vec::new();
    for _i in 0..10 {
        let mut p = Particle::new();
        p
            .set_position(Vector { x: random(), y: random() })
            .set_velocity(Vector { x: random() - 5.0, y: random() - 5.0 });
        particles.push(p);
    }

    let mut sim_data = SimData::new(0., 10., 0., 10.);

    for particle in particles {
        sim_data.add_particle(&particle);
    }

    let verlet_lists = create_verlet_lists(&sim_data, 0.1);
    println!("Number of lists in the verlet lists: {}", verlet_lists.verlet_lists.len());

    for (id, neighbors) in verlet_lists.verlet_lists {

    }

    let mut x = Vec::new();
    let mut y = Vec::new();
    for i in 0..sim_data.num_particles() {
        x.push(sim_data.positions[i as usize].x);
        y.push(sim_data.positions[i as usize].y);
    }
    let trace = Scatter::new(x, y)
        .name("My trace")
        .mode(Mode::Markers);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    let layout = Layout::new().title("<b>Line and Scatter Plot</b>".into());
    plot.set_layout(layout);
    plot.show();
}
