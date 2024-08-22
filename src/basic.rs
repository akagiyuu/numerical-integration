use rayon::iter::{ParallelBridge, ParallelIterator};

#[allow(clippy::redundant_closure)]
fn _left_end_point(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + Send + Sync + Copy,
) -> impl Fn(usize) -> f64 {
    move |n| {
        assert!(n > 0);

        if a == b {
            return 0.;
        }

        let step = (b - a) / n as f64;
        (0..n)
            .par_bridge()
            .map(|i| a + i as f64 * step)
            .map(|x_i| f(x_i))
            .sum::<f64>()
            * step
    }
}

pub fn left_end_point(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + Send + Sync + Copy,
    partition_count: usize,
) -> f64 {
    let approximation_f = _left_end_point(a, b, f);

    approximation_f(partition_count)
}

#[allow(clippy::redundant_closure)]
fn _right_end_point(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + Send + Sync + Copy,
) -> impl Fn(usize) -> f64 {
    move |n| {
        assert!(n > 0);

        if a == b {
            return 0.;
        }

        let step = (b - a) / n as f64;
        (1..=n)
            .par_bridge()
            .map(|i| a + i as f64 * step)
            .map(|x_i| f(x_i))
            .sum::<f64>()
            * step
    }
}

pub fn right_end_point(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + Send + Sync + Copy,
    partition_count: usize,
) -> f64 {
    let approximation_f = _left_end_point(a, b, f);

    approximation_f(partition_count)
}
