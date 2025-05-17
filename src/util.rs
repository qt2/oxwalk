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
#[allow(unused)]
pub fn nearest_point_on_line_segment(point: DVec2, line: [DVec2; 2]) -> DVec2 {
    let p0 = line[0];
    let p1 = line[1];
    let seg = p1 - p0;
    let seg_len_sq = seg.length_squared();
    if seg_len_sq == 0.0 {
        return p0;
    }
    let t = ((point - p0).dot(seg) / seg_len_sq).clamp(0.0, 1.0);

    p0 + seg * t
}

/// Calculate the nearest point on a path defined by a series of points to a given point.
pub fn nearest_point_on_path(point: DVec2, path: &[DVec2]) -> DVec2 {
    assert!(!path.is_empty());
    let mut nearest_point = path[0];
    let mut min_dist_sq = (point - nearest_point).length_squared();

    for i in 1..path.len() {
        let p0 = path[i - 1];
        let p1 = path[i];
        let seg = p1 - p0;
        let seg_len_sq = seg.length_squared();
        if seg_len_sq == 0.0 {
            continue;
        }
        let t = ((point - p0).dot(seg) / seg_len_sq).clamp(0.0, 1.0);
        let proj = p0 + seg * t;
        let dist_sq = (point - proj).length_squared();
        if dist_sq < min_dist_sq {
            min_dist_sq = dist_sq;
            nearest_point = proj;
        }
    }

    nearest_point
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
        assert!((nearest_point - dvec2(0.5, 0.5)).length() < 1e-10);
    }

    #[test]
    fn test_nearest_point_on_path() {
        let path = vec![dvec2(0.0, 0.0), dvec2(1.0, 1.0), dvec2(0.0, 2.0)];
        let point = dvec2(0.1, 1.1);
        let nearest_point = nearest_point_on_path(point, &path);
        assert!((nearest_point - dvec2(0.5, 1.5)).length() < 1e-10);
    }
}
