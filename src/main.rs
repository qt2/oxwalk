mod model;
mod util;
mod visualizer;

use glam::dvec2;
use model::{Destination, Obstacle, Pedestrian, State};
use visualizer::Visualizer;

fn main() {
    std::fs::create_dir("output").ok();
    let output_path = chrono::Local::now()
        .format("output/%Y-%m-%d_%H-%M-%S.gif")
        .to_string();
    let mut visualizer =
        Visualizer::new(&output_path, [dvec2(0.0, 0.0), dvec2(10.0, 4.0)], 64.0, 100);

    let mut state = State::default();

    // Destination #0
    state.add_destination(Destination::new(vec![dvec2(1.0, 1.0), dvec2(1.0, 3.0)]));

    // Destination #1
    state.add_destination(Destination::new(vec![dvec2(9.0, 1.0), dvec2(9.0, 3.0)]));

    // Obstacles
    state.add_obstacle(Obstacle::new(vec![dvec2(0.0, 0.0), dvec2(10.0, 0.0)]));
    state.add_obstacle(Obstacle::new(vec![dvec2(0.0, 4.0), dvec2(10.0, 4.0)]));
    state.add_obstacle(Obstacle::new(vec![
        dvec2(5.0, 1.0),
        dvec2(6.0, 2.0),
        dvec2(5.0, 3.0),
        dvec2(4.0, 2.0),
        dvec2(5.0, 1.0),
    ]));

    for step in 0..1000 {
        for _ in 0..util::poisson(0.1) {
            state.spawn_pedestrian(Pedestrian {
                position: dvec2(1.0, 1.0 + fastrand::f64() * 2.0),
                destination_id: 1,
                ..Default::default()
            });
        }
        for _ in 0..util::poisson(0.1) {
            state.spawn_pedestrian(Pedestrian {
                position: dvec2(9.0, 1.0 + fastrand::f64() * 2.0),
                destination_id: 0,
                ..Default::default()
            });
        }

        state.tick();

        if step % 2 == 0 {
            visualizer.render(step, &state);
        }
        if step % 100 == 0 {
            let active_pedestrians = state.pedestrians.iter().filter(|p| p.active).count();
            println!("Step {step}: {active_pedestrians} pedestrians");
        }
    }

    println!("Simulation finished. Output saved to {output_path}");
}
