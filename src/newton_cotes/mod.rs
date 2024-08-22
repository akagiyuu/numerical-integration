mod closed;
mod open;

use closed::closed_newton_cotes;
use itertools::Itertools;
use open::open_newton_cotes;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

#[derive(Debug, Clone, Copy)]
pub enum PointType {
    Open,
    Closed,
}

pub fn newton_cotes(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + Send + Sync + Copy,
    n: usize,
    point_type: PointType,
) -> f64 {
    match point_type {
        PointType::Open => open_newton_cotes(a, b, f, n),
        PointType::Closed => closed_newton_cotes(a, b, f, n),
    }
}

pub fn composite_newton_cotes(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + Send + Sync + Copy,
    n: usize,
    point_type: PointType,
    partition_count: usize,
) -> f64 {
    if a == b {
        return 0.;
    }

    let step = (b - a) / partition_count as f64;
    (0..partition_count + 1)
        .map(|i| a + step * i as f64)
        .tuple_windows::<(_, _)>()
        .par_bridge()
        .map(|(a_i, b_i)| newton_cotes(a_i, b_i, f, n, point_type))
        .sum()
}

pub fn adaptive_newton_cotes(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + Send + Sync + Copy,
    n: usize,
    point_type: PointType,
    error: f64,
) -> f64 {
    assert!(error > 0.);

    if a == b {
        return 0.;
    }

    let total = newton_cotes(a, b, f, n, point_type);
    let mid_point = (a + b) / 2.;
    let left = newton_cotes(a, mid_point, f, n, point_type);
    let right = newton_cotes(mid_point, b, f, n, point_type);

    let delta = left + right - total;

    if delta.abs() < error {
        left + right
    } else {
        [(a, mid_point), (mid_point, b)]
            .into_par_iter()
            .map(|(left_point, right_point)| {
                adaptive_newton_cotes(left_point, right_point, f, n, point_type, error / 2.)
            })
            .sum()
    }
}
