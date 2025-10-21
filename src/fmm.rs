use std::collections::BinaryHeap;

use ndarray::Array2;

struct Cell {
    u: f64,
    index: (usize, usize),
}

impl Cell {
    fn new(u: f64, index: (usize, usize)) -> Self {
        Cell { u, index }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.u.eq(&other.u)
    }
}

impl Eq for Cell {}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reverse ordering for picking smallest item with `pop`
        other.u.partial_cmp(&self.u)
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

pub fn apply_fmm(field: &mut Array2<f64>, scale_factor: f64, n: impl Fn((usize, usize)) -> f64) {
    let dim = field.dim();
    let mut fixed = Array2::from_elem(dim, false);

    let index_add = |index: (usize, usize), delta: (isize, isize)| match (
        index.0.checked_add_signed(delta.0),
        index.1.checked_add_signed(delta.1),
    ) {
        (Some(y), Some(x)) if y < dim.0 && x < dim.1 => Some((y, x)),
        _ => None,
    };

    let mut heap = BinaryHeap::new();

    for (index, &u) in field.indexed_iter() {
        if u == 0.0 {
            heap.push(Cell::new(u, index));
        }
    }

    while let Some(Cell { index, .. }) = heap.pop() {
        if fixed[index] {
            continue;
        }

        fixed[index] = true;

        for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let Some(index_n) = index_add(index, delta) else {
                continue;
            };

            if fixed[index_n] || field[index_n].is_nan() {
                continue;
            }

            let neighbor = |delta: (isize, isize)| {
                index_add(index_n, delta)
                    .map(|i| {
                        let v = field[i];
                        if v.is_nan() { f64::INFINITY } else { v }
                    })
                    .unwrap_or(f64::INFINITY)
            };

            let ux = neighbor((-1, 0)).min(neighbor((1, 0)));
            let uy = neighbor((0, -1)).min(neighbor((0, 1)));

            let n = scale_factor * n(index_n);

            let u_candidate = if ux == f64::INFINITY {
                uy + n
            } else if uy == f64::INFINITY {
                ux + n
            } else {
                let sq = 2.0 * n * n - (ux - uy).powi(2);
                if sq >= 0.0 {
                    (ux + uy + sq.sqrt()) * 0.5
                } else {
                    ux.min(uy) + n
                }
            };

            if u_candidate < field[index_n] {
                field[index_n] = u_candidate;
                heap.push(Cell::new(u_candidate, index_n));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ndarray::s;

    use super::*;

    #[test]
    fn test_fmm() {
        let mut field = Array2::from_elem((10, 10), f64::NAN);
        field.slice_mut(s![1..9, 1..9]).fill(f64::INFINITY);
        field.slice_mut(s![4..6, 4..6]).fill(0.0);

        apply_fmm(&mut field, 0.25, |_| 1.0);

        println!("{field:?}");
    }
}
