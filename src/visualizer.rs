use glam::DVec2;
use plotters::{coord::Shift, prelude::*, style::full_palette::GREY_500};

use crate::model::State;

const COLORS: &[RGBColor] = &[BLUE, RED, GREEN, YELLOW, CYAN, MAGENTA];

/// Visualizer for the simulation.
///
/// This struct is responsible for rendering the simulation state to a GIF file.
pub struct Visualizer<'a> {
    canvas: DrawingArea<BitMapBackend<'a>, Shift>,
    render_rect: [DVec2; 2],
    resolution: f64,
}

impl<'a> Visualizer<'a> {
    /// Creates a new visualizer.
    ///
    /// - `output_path`: Path to the output GIF file.
    /// - `render_rect`: The rectangle in world coordinates that will be rendered.
    /// - `resolution`: The number of pixels per unit in the render rectangle.
    /// - `frame_delay`: Delay between frames in milliseconds.
    pub fn new(
        output_path: &str,
        render_rect: [DVec2; 2],
        resolution: f64,
        frame_delay: u32,
    ) -> Self {
        let image_size = ((render_rect[1] - render_rect[0]) * resolution).as_uvec2();
        let canvas = BitMapBackend::gif(output_path, image_size.into(), frame_delay)
            .unwrap()
            .into_drawing_area();
        Self {
            canvas,
            render_rect,
            resolution,
        }
    }

    /// Renders the current state of the simulation.
    pub fn render(&mut self, _step: i32, state: &State) {
        self.canvas.fill(&WHITE).unwrap();

        for obstacle in &state.obstacles {
            let style = GREY_500.stroke_width((0.25 * self.resolution) as u32);
            self.draw_path(&obstacle.points, style);
        }

        for destination in &state.destinations {
            let style = BLACK.stroke_width((0.25 * self.resolution) as u32);
            self.draw_path(&destination.points, style);
        }

        for pedestrian in &state.pedestrians {
            if pedestrian.active {
                let color = COLORS[pedestrian.destination_id as usize % COLORS.len()];
                self.draw_circle(pedestrian.position, 0.2, color.filled());
            }
        }

        self.canvas.present().unwrap();
    }

    /// Draws a circle at the given position with the specified radius and style.
    /// The position is in world coordinates, and the radius is in world units.
    fn draw_circle(&self, position: DVec2, radius: f64, style: impl Into<ShapeStyle>) {
        let coord = ((position - self.render_rect[0]) * self.resolution).as_ivec2();
        self.canvas
            .draw(&Circle::new(
                coord.into(),
                (radius * self.resolution) as i32,
                style,
            ))
            .unwrap();
    }

    /// Draws a path defined by a series of points.
    /// The points are in world coordinates.
    fn draw_path(&self, points: &[DVec2], style: impl Into<ShapeStyle>) {
        let points: Vec<(i32, i32)> = points
            .iter()
            .map(|v| {
                ((v - self.render_rect[0]) * self.resolution)
                    .as_ivec2()
                    .into()
            })
            .collect();
        self.canvas.draw(&PathElement::new(points, style)).unwrap();
    }
}
