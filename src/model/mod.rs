use glam::DVec2;

use crate::util;

/// Simulation time step in seconds.
pub const TIME_STEP: f64 = 1.0 / 10.0;
/// Cosine of phi (2*phi represents the effective angle of sight of pedestrians).
const COS_PHI: f64 = -0.17364817766693036; // cos(100 degrees)

#[derive(Debug, Default)]
pub struct State {
    pub pedestrians: Vec<Pedestrian>,
    pub obstacles: Vec<Obstacle>,
    pub destinations: Vec<Destination>,
}

impl State {
    pub fn spawn_pedestrian(&mut self, pedestrian: Pedestrian) {
        self.pedestrians.push(pedestrian);
    }

    pub fn add_obstacle(&mut self, obstacle: Obstacle) {
        self.obstacles.push(obstacle);
    }

    pub fn add_destination(&mut self, destination: Destination) {
        self.destinations.push(destination);
    }

    pub fn tick(&mut self) {
        let pedestrians = &mut self.pedestrians;

        let accelerations = (0..pedestrians.len())
            .map(|self_id| {
                let self_p = &pedestrians[self_id];
                let mut acceleration = DVec2::ZERO;

                let target = util::nearest_point_on_line_segment(
                    self_p.position,
                    self.destinations[self_p.destination_id].points,
                );

                let desired_move_dir = (target - self_p.position).normalize_or_zero();
                acceleration += (desired_move_dir * self_p.desired_speed - self_p.velocity) / 0.5;

                for other_id in 0..pedestrians.len() {
                    if self_id != other_id {
                        let other_p = &pedestrians[other_id];
                        if !other_p.active {
                            continue;
                        }

                        let diff = self_p.position - other_p.position;
                        let distance = diff.length();

                        if distance > 0.0 && distance <= 3.0 {
                            let direction = diff / distance;
                            let t1 = diff - other_p.velocity * TIME_STEP;
                            let t1_length = t1.length();
                            let t2 = distance + t1_length;

                            let b = (t2.powi(2) - (other_p.velocity.length() * TIME_STEP).powi(2))
                                .sqrt()
                                * 0.5;
                            let nabla_b = t2 * (direction + t1 / t1_length) / (4.0 * b);
                            let mut force = 2.1 / 0.3 * (-b / 0.3).exp() * nabla_b;

                            if desired_move_dir.dot(-force) < force.length() * COS_PHI {
                                force *= 0.5;
                            }

                            acceleration += force;
                        }
                    }
                }

                for obstacle in &self.obstacles {
                    let nearest_point =
                        util::nearest_point_on_line_segment(self_p.position, obstacle.points);
                    let diff = self_p.position - nearest_point;
                    let distance = diff.length();

                    if distance > 0.0 && distance <= 3.0 {
                        let direction = diff / distance;
                        let force = 10.0 * 0.2 * (-distance / 0.2).exp() * direction;
                        acceleration += force;
                    }
                }

                acceleration
            })
            .collect::<Vec<_>>();

        for (i, pedestrian) in pedestrians.iter_mut().enumerate() {
            if pedestrian.active {
                let velocity_current = pedestrian.velocity;
                pedestrian.velocity += accelerations[i] * TIME_STEP;
                pedestrian.velocity = pedestrian
                    .velocity
                    .clamp_length_max(pedestrian.desired_speed * 1.3);
                pedestrian.position += (pedestrian.velocity + velocity_current) * TIME_STEP * 0.5;

                let target = util::nearest_point_on_line_segment(
                    pedestrian.position,
                    self.destinations[pedestrian.destination_id].points,
                );
                let distance = (target - pedestrian.position).length_squared();
                if distance < 0.2f64.powi(2) {
                    pedestrian.active = false;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pedestrian {
    /// Active flag.
    /// If false, the pedestrian is not considered in the simulation.
    pub active: bool,
    /// Position.
    pub position: DVec2,
    /// Velocity.
    pub velocity: DVec2,
    /// Desired speed.
    pub desired_speed: f64,
    /// Destination ID.
    pub destination_id: usize,
}

impl Default for Pedestrian {
    fn default() -> Self {
        Self {
            active: true,
            position: DVec2::ZERO,
            velocity: DVec2::ZERO,
            desired_speed: fastrand_contrib::f64_normal_approx(1.34, 0.26),
            destination_id: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Obstacle {
    /// Line segment representing the obstacle.
    pub points: [DVec2; 2],
}

impl Obstacle {
    pub fn from_line_segment(points: [DVec2; 2]) -> Self {
        Obstacle { points }
    }
}

#[derive(Debug, Clone)]
pub struct Destination {
    pub points: [DVec2; 2],
}

impl Destination {
    pub fn from_line_segment(points: [DVec2; 2]) -> Self {
        Destination { points }
    }
}
