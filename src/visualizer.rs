use glam::DVec2;
use plotters::{coord::Shift, prelude::*, style::full_palette::GREY_500};

use crate::model::State;

const COLORS: &[RGBColor] = &[BLUE, RED, GREEN, YELLOW, CYAN, MAGENTA];

pub struct Visualizer<'a> {
    root: DrawingArea<BitMapBackend<'a>, Shift>,
    render_rect: [DVec2; 2],
    resolution: f64,
}

impl<'a> Visualizer<'a> {
    pub fn new(
        output_path: &str,
        render_rect: [DVec2; 2],
        resolution: f64,
        frame_delay: u32,
    ) -> Self {
        let image_size = ((render_rect[1] - render_rect[0]) * resolution).as_uvec2();
        let root = BitMapBackend::gif(output_path, image_size.into(), frame_delay)
            .unwrap()
            .into_drawing_area();
        Self {
            root,
            render_rect,
            resolution,
        }
    }

    pub fn render(&mut self, _step: i32, state: &State) {
        let canvas = &mut self.root;
        canvas.fill(&WHITE).unwrap();

        for obstacle in &state.obstacles {
            let points: Vec<_> = obstacle
                .vertices
                .iter()
                .map(|v| {
                    ((v - self.render_rect[0]) * self.resolution)
                        .as_ivec2()
                        .into()
                })
                .collect();

            canvas
                .draw(&PathElement::new(
                    points,
                    GREY_500.stroke_width((0.25 * self.resolution) as u32),
                ))
                .unwrap();
        }

        for destination in &state.destinations {
            let points: Vec<_> = destination
                .vertices
                .iter()
                .map(|v| {
                    ((v - self.render_rect[0]) * self.resolution)
                        .as_ivec2()
                        .into()
                })
                .collect();

            canvas
                .draw(&PathElement::new(
                    points,
                    BLACK.stroke_width((0.25 * self.resolution) as u32),
                ))
                .unwrap();
        }

        for pedestrian in &state.pedestrians {
            if pedestrian.active {
                let coord =
                    ((pedestrian.position - self.render_rect[0]) * self.resolution).as_ivec2();
                let color = COLORS[pedestrian.destination_id as usize % COLORS.len()];

                canvas
                    .draw(&Circle::new(
                        coord.into(),
                        (0.2 * self.resolution) as i32,
                        color.filled(),
                    ))
                    .unwrap();
            }
        }

        self.root.present().unwrap();
    }
}
