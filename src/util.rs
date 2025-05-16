use glam::DVec2;

/// Spawn a random integer based on Poisson distribution.
pub fn poisson(lambda: f64) -> i32 {
    let mut y = 0;
    let mut x = fastrand::f64();
    let exp_lambda = (-lambda).exp();
    while x >= exp_lambda {
        x *= fastrand::f64();
        y += 1;
    }

    y
}

/// Calculate the nearest point on a line segment to a given point.
pub fn nearest_point_on_line_segment(point: DVec2, line: [DVec2; 2]) -> DVec2 {
    let line_vec = line[1] - line[0];
    let line_length_squared = line_vec.length_squared();
    if line_length_squared == 0.0 {
        return line[0];
    }

    let t = ((point - line[0]).dot(line_vec)) / line_length_squared;
    if t < 0.0 {
        return line[0];
    } else if t > 1.0 {
        return line[1];
    }

    line[0] + t * line_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::dvec2;

    #[test]
    fn test_nearest_point_on_line_segment() {
        let line = [dvec2(0.0, 0.0), dvec2(1.0, 1.0)];
        let point = dvec2(1.0, 0.0);
        let nearest_point = nearest_point_on_line_segment(point, line);
        assert_eq!(nearest_point, dvec2(0.5, 0.5));
    }
}
