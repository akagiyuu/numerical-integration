use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::closed_newton_cotes;

fn _simpson(a: f64, b: f64, f: impl Fn(f64) -> f64 + Send + Sync + Copy) -> f64 {
    closed_newton_cotes(a, b, f, 3, 1)
}

pub fn adaptive_simpson(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + Send + Sync + Copy,
    error: f64,
) -> f64 {
    assert!(error > 0.);

    if a == b {
        return 0.;
    }

    let total = _simpson(a, b, f);
    let mid_point = (a + b) / 2.;
    let left = _simpson(a, mid_point, f);
    let right = _simpson(mid_point, b, f);

    let delta = left + right - total;

    if delta.abs() < 15. * error {
        left + right + delta / 15.
    } else {
        [(a, mid_point), (mid_point, b)]
            .into_par_iter()
            .map(|(left_point, right_point)| {
                adaptive_simpson(left_point, right_point, f, error / 2.)
            })
            .sum()
    }
}
